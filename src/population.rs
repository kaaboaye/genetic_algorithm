use crate::consts::Number;
use na::{DMatrix, DVector};
use rand::distributions::{Uniform, Distribution};
use rand::thread_rng;
use crate::scenario::Scenario;
use crossbeam_utils::thread;

#[derive(Debug)]
pub struct Population {
    population: DMatrix<Number>,
}

impl Population {
    pub fn new(population_size: usize, number_of_objects: usize) -> Population {
        let mut rng = thread_rng();
        let bool_int = Uniform::from(0..2 as Number);

        // TODO bottle neck
        let population = DMatrix::<Number>::from_fn(
            population_size,
            number_of_objects,
            |_, _| bool_int.sample(&mut rng),
        );

        Population { population }
    }

    pub fn evaluate(&self, scenario: &Scenario) -> DVector<Number> {
        thread::scope(|scope| {
            let weights_thread = scope.spawn(|_| {
                let mut weights = &self.population * &scenario.weights;
                for elem in weights.iter_mut() {
                    *elem = (*elem <= scenario.max_weight) as Number;
                }

                weights
            });

            let sizes_thread = scope.spawn(|_| {
                let mut sizes = &self.population * &scenario.sizes;
                for elem in sizes.iter_mut() {
                    *elem = (*elem <= scenario.max_size) as Number;
                }

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
