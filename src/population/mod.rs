pub mod config;
mod individual;

use crate::consts::Number;
use crate::population::config::Config;
use crate::population::individual::new_individual;
use crate::scenario::Scenario;
use crossbeam_utils::thread;
use na::{DMatrix, DVector};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use rayon::prelude::*;
use std::mem::swap;

#[derive(Debug)]
pub struct Population {
    scenario: Scenario,
    config: Config,
    generation_count: usize,
    population: Box<DMatrix<Number>>,
    next_population: Box<DMatrix<Number>>,
}

#[derive(Debug)]
struct EvaluationArena {
    weights: DVector<Number>,
    sizes: DVector<Number>,
    costs: DVector<Number>,
}

impl Population {
    pub fn new(scenario: Scenario, config: Config) -> Population {
        let population = Box::new(generate_random_population(
            config.population_size,
            scenario.number_of_objects as usize,
        ));

        // it is save because next_populations is only allocated memory placeholder
        let next_population = Box::new(unsafe {
            DMatrix::<Number>::new_uninitialized(
                config.population_size,
                scenario.number_of_objects as usize,
            )
        });

        Population {
            population,
            next_population,
            scenario,
            config,
            generation_count: 0,
        }
    }

    pub fn evolve(&mut self) -> Number {
        let best_individual = generate_evolved_population(
            &self.population,
            self.next_population.as_mut(),
            &self.scenario,
            &self.config,
        );

        swap(self.population.as_mut(), self.next_population.as_mut());

        best_individual
    }
}

/// Generates and returns random population of given size
fn generate_random_population(population_size: usize, number_of_objects: usize) -> DMatrix<Number> {
    let bool_int = Uniform::from(0..11 as Number);

    let vec = (0..(population_size * number_of_objects))
        .into_par_iter()
        .map_init(
            || thread_rng(),
            |mut rng, _| (bool_int.sample(&mut rng) == 1) as Number,
        )
        .collect();

    DMatrix::<Number>::from_vec(population_size, number_of_objects, vec)
}

fn generate_evolved_population(
    population: &DMatrix<Number>,
    next_population: &mut DMatrix<Number>,
    scenario: &Scenario,
    population_config: &Config,
) -> Number {
    let scores = evaluate_population(population, scenario);

    let best_score = *scores.data.as_vec().par_iter().max().unwrap();

    // chunk population by each individual
    next_population
        .row_iter_mut()
        .collect::<Vec<_>>()
        .par_iter_mut()
        .for_each_init(
            || thread_rng(),
            |rng, mut child| {
                let parent1 = tournament(&scores, population_config.tournament_size);
                let parent2 = tournament(&scores, population_config.tournament_size);

                let parent1 = population.row(parent1);
                let parent2 = population.row(parent2);

                new_individual(&mut child, &parent1, &parent2, rng, population_config);
            },
        );

    best_score
}

fn evaluate_population(population: &DMatrix<Number>, scenario: &Scenario) -> DVector<Number> {
    thread::scope(|scope| {
        let weights_thread = scope.spawn(|_| {
            // calculate weights for population individuals
            let mut weights = population * &scenario.weights;

            // select only individuals matching the requirements
            // it is save because vec len stays the same
            let vec = unsafe { weights.data.as_vec_mut() };
            vec.into_par_iter().for_each(|elem| {
                *elem = (*elem <= scenario.max_weight) as Number;
            });

            weights
        });

        let sizes_thread = scope.spawn(|_| {
            // calculate sizes for population individuals
            let mut sizes = population * &scenario.sizes;

            // select only individuals matching the requirements
            // it is save because vec len stays the same
            let vec = unsafe { sizes.data.as_vec_mut() };
            vec.into_par_iter().for_each(|elem| {
                *elem = (*elem <= scenario.max_size) as Number;
            });

            sizes
        });

        let costs_thread = scope.spawn(|_| population * &scenario.costs);

        let weights = weights_thread.join().unwrap();
        let sizes = sizes_thread.join().unwrap();
        let mut costs = costs_thread.join().unwrap();

        costs.component_mul_assign(&weights);
        costs.component_mul_assign(&sizes);

        costs
    })
    .unwrap()
}

/// Selects individual using tournament algorithm
/// Returns selected individual's index
fn tournament(scores: &DVector<Number>, tournament_size: usize) -> usize {
    let mut selector = random_vec(tournament_size, scores.nrows());

    // Filter selected individuals
    selector.component_mul_assign(scores);
    let (best_idx, _) = selector.argmax();
    best_idx
}

/// Returns DVector of zeros and ones.
/// It will contain randomly distributed `desired_positives` of ones (1).
/// The rest of values will be 0.
fn random_vec(desired_positives: usize, size: usize) -> DVector<Number> {
    let res = if desired_positives == 0 {
        // fast path for vector full of 0
        vec![0; size]
    } else if desired_positives == size {
        // fast path for vector full of 1
        vec![1; size]
    } else if desired_positives <= size / 2 {
        // generate sparse vector
        sparse_random_vec(desired_positives, size)
    } else {
        // generate dense vector
        //
        // In order to avoid large number of collisions create sparse negation and then
        // and then negate the vector back.
        let mut res = sparse_random_vec(size - desired_positives, size);

        for num in res.iter_mut() {
            *num = *num ^ (1 as Number);
        }

        res
    };

    DVector::<Number>::from_vec(res)
}

fn sparse_random_vec(desired_positives: usize, size: usize) -> Vec<Number> {
    // setting desired positions to zero will cause and infinite loop
    // use vec![0, size] instead
    debug_assert_ne!(desired_positives, 0);

    let mut rng = thread_rng();
    let mut res: Vec<Number> = vec![0; size];
    let mut positives: usize = 0;

    let slots = Uniform::from(0..size);

    loop {
        let idx = slots.sample(&mut rng);

        // It will always increment the number of positives at first
        // then it will subtract value of given position.
        //
        // This subtraction handles collisions.
        //
        // It is equivalent to
        // `if res[idx] == 0 { positives += 1 }`
        // but this is faster because no branching is happening in this implementation.
        positives += 1;
        positives -= res[idx] as usize;

        res[idx] = 1;

        if positives == desired_positives {
            break;
        }
    }

    res
}
