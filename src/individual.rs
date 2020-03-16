use crate::consts::Number;
use rand::Rng;
use rand::prelude::ThreadRng;
use crate::population_config::PopulationConfig;

type Individual<'a> = na::Matrix<
    Number,
    na::U1,
    na::Dynamic,
    na::SliceStorage<'a, Number, na::U1, na::Dynamic, na::U1, na::Dynamic>
>;

pub fn new_individual(
    parent1: &Individual,
    parent2: &Individual,
    rng: &mut ThreadRng,
    config: &PopulationConfig,
) -> Vec<Number> {
    let mut individual = crossover(parent1, parent2, rng, config);
    mutate(&mut individual, rng, config);
    individual
}

fn crossover(
    parent1: &Individual,
    parent2: &Individual,
    rng: &mut ThreadRng,
    config: &PopulationConfig,
) -> Vec<Number> {
    let parent1 = parent1.iter().map(|n| *n);
    let parent2 = parent2.iter().map(|n| *n);

    if config.crossover_probability < rng.gen::<f64>() {
        return parent1.collect();
    }

    parent1
        .take(config.crossover_portion)
        .chain(parent2.skip(config.crossover_portion))
        .collect()
}

fn mutate(individual: &mut Vec<Number>, rng: &mut ThreadRng, config: &PopulationConfig) {
    for gen in individual.iter_mut() {
        *gen = *gen ^ (config.mutation_probability >= rng.gen::<f64>()) as Number;
    }
}
