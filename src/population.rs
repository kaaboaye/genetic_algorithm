use crate::consts::Number;
use na::DMatrix;
use rand::distributions::{Uniform, Distribution};
use rand::thread_rng;

#[derive(Debug)]
pub struct Population {
    pub data: DMatrix<Number>
}

impl Population {
    pub fn new(population_size: usize, number_of_objects: usize) -> Population {
        let mut rng = thread_rng();
        let bool_int = Uniform::from(0..2 as Number);

        let data = DMatrix::<Number>::from_fn(
            number_of_objects,
            population_size,
            |_, _| bool_int.sample(&mut rng),
        );

        Population { data }
    }
}
