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
}
