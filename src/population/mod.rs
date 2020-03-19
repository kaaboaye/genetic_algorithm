pub mod config;
mod individual;
mod random_vec;

use crate::consts::Number;
use crate::population::config::Config;
use crate::population::individual::new_individual;
use crate::population::random_vec::random_vec;
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
    population: DMatrix<Number>,
    next_population: DMatrix<Number>,
}

#[derive(Debug)]
struct EvaluationArena {
    weights: DVector<Number>,
    sizes: DVector<Number>,
    costs: DVector<Number>,
}

impl Population {
    pub fn new(scenario: Scenario, config: Config) -> Population {
        let population =
            generate_random_population(config.population_size, scenario.number_of_objects as usize);

        // it is save because next_populations is only allocated memory placeholder
        let next_population = unsafe {
            DMatrix::<Number>::new_uninitialized(
                config.population_size,
                scenario.number_of_objects as usize,
            )
        };

        Population {
            population,
            next_population,
            scenario,
            config,
        }
    }

    pub fn evolve(&mut self) -> Number {
        let best_individual = evolve_population(
            &self.population,
            &mut self.next_population,
            &self.scenario,
            &self.config,
        );

        // it is save because both matrices have the same size
        unsafe {
            swap(
                self.population.data.as_vec_mut(),
                self.next_population.data.as_vec_mut(),
            );
        }

        best_individual
    }
}

/// Generates and returns random population of given size
fn generate_random_population(population_size: usize, number_of_objects: usize) -> DMatrix<Number> {
    // It creates a vector containing genes for feature population.
    // This vector is being created using multiple threads.

    let bool_int = Uniform::from(0..11 as Number);

    let vec = (0..(population_size * number_of_objects))
        // creates a parallel iterator
        .into_par_iter()
        .map_init(
            // initializes state for each thread
            || thread_rng(),
            // evaluates value for each gene
            |mut rng, _| (bool_int.sample(&mut rng) == 1) as Number,
        )
        .collect();

    DMatrix::<Number>::from_vec(population_size, number_of_objects, vec)
}

/// Evolves population and stores the result in `next_population`.
///
/// Returns the `best_score` before the evolution.
fn evolve_population(
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

/// Evaluates the population
///
/// Returns a vector of scores
fn evaluate_population(population: &DMatrix<Number>, scenario: &Scenario) -> DVector<Number> {
    // This function evaluates population using 3 independent threads.
    //
    // The first one calculates weights for each individual and checks the requirements.
    // Returns a boolean vector indicating whether given which individuals
    // are fulfilling the requirements.
    //
    // The second one calculates sizes and also checks the requirements. And also returns
    // a boolean vector.
    //
    // And the last one calculates costs. Returns a vector indicating cost of each individual
    // in the population.
    //
    // After all 3 threads join costs are filtered using weight and size boolean vectors.

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

        // Filter out individuals not meeting the requirements.
        costs.component_mul_assign(&weights);
        costs.component_mul_assign(&sizes);

        costs
    })
    .unwrap()
}

/// Selects individual using tournament algorithm
/// Returns selected individual's index
fn tournament(scores: &DVector<Number>, tournament_size: usize) -> usize {
    // Creates a random vector of {0, 1} and multiplies value by score.
    // Them finds the best value and returns index of chosen individual.

    let mut selector = random_vec(tournament_size, scores.nrows());

    // Filter selected individuals
    selector.component_mul_assign(scores);
    let (best_idx, _) = selector.argmax();
    best_idx
}
