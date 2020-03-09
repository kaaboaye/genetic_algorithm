use crate::consts::Number;
use na::{DMatrix, DVector};
use rand::distributions::{Uniform, Distribution};
use rand::thread_rng;
use crate::scenario::Scenario;
use crossbeam_utils::thread;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Population {
    population: DMatrix<Number>,
}

impl Population {
    pub fn new(population_size: usize, number_of_objects: usize) -> Population {
        let bool_int = Uniform::from(0..2 as Number);

        let vec = (0..(population_size * number_of_objects))
            .into_par_iter()
            .map_init(
                || thread_rng(),
                |mut rng, _| bool_int.sample(&mut rng),
            )
            .collect();

        let population = DMatrix::<Number>::from_vec(
            population_size,
            number_of_objects,
            vec,
        );

        Population { population }
    }

    pub fn evaluate(&self, scenario: &Scenario) -> DVector<Number> {
        thread::scope(|scope| {
            let weights_thread = scope.spawn(|_| {
                let mut weights = &self.population * &scenario.weights;
                let vec = unsafe { weights.data.as_vec_mut() };
                vec.into_par_iter().for_each(|elem| {
                    *elem = (*elem <= scenario.max_weight) as Number;
                });

                weights
            });

            let sizes_thread = scope.spawn(|_| {
                let mut sizes = &self.population * &scenario.sizes;
                let vec = unsafe { sizes.data.as_vec_mut() };
                vec.into_par_iter().for_each(|elem| {
                    *elem = (*elem <= scenario.max_size) as Number;
                });

                sizes
            });

            let costs_thread = scope.spawn(|_| &self.population * &scenario.costs);

            let weights = weights_thread.join().unwrap();
            let sizes = sizes_thread.join().unwrap();
            let mut costs = costs_thread.join().unwrap();

            costs.component_mul_assign(&weights);
            costs.component_mul_assign(&sizes);

            costs
        }).unwrap()
    }

    pub fn tournament(&self, scores: &DVector<Number>, tournament_size: usize) -> usize {
        let mut selector = random_vec(
            tournament_size,
            self.population.nrows()
        );

        // Filter selected individuals
        selector.component_mul_assign(scores);
        let (best_idx, _) = selector.argmax();
        best_idx
    }
}

/// Returns DVector of zeros and ones.
/// It will contain randomly distributed `desired_positives` of ones (1).
/// The rest of values will be 0.
/// It assumes that `desired_positives` is greater then 0.
fn random_vec(desired_positives: usize, size: usize) -> DVector<Number> {
    debug_assert_ne!(desired_positives, 0);

    let mut rng = thread_rng();
    let mut res: Vec<Number> = vec![0; size];
    let mut positives: usize = 0;

    let slots = Uniform::from(0..size);

    loop {
        let idx = slots.sample(&mut rng);

        // If position is already chosen it will subtract 1
        // which will be added back later.
        // If position was not used before it will subtract 0
        // which later will be incremented by 1 increasing
        // the total number of selected slots.
        //
        // It is equivalent to
        // `if res[idx] == 0 { positives += 1 }`
        // but this will be faster because no branching is happening in this implementation.
        //
        // Since it operates on unsigned number it takes an assumption that
        // `positives -= res[idx]` will never produce a negative result.
        // This assumption is true because `positives` has to have at least value of `1`
        // in order to some conflict to occur.
        // In other words positives cannot find duplicate if there are no positive values.
        positives += 1;
        positives -= res[idx] as usize;

        res[idx] = 1;

        if positives == desired_positives { break; }
    }

    DVector::<Number>::from_vec(res)
}
