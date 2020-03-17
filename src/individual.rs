use crate::consts::Number;
use crate::population_config::PopulationConfig;
use rand::prelude::ThreadRng;
use rand::Rng;

type Individual<'a> = na::Matrix<
    Number,
    na::U1,
    na::Dynamic,
    na::SliceStorage<'a, Number, na::U1, na::Dynamic, na::U1, na::Dynamic>,
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
    let parent1_iter = parent1.iter().map(|n| *n);
    let parent2_iter = parent2.iter().map(|n| *n);

    if config.crossover_probability < rng.gen::<f64>() {
        return parent1_iter.collect();
    }

    // rng.gen_range generates [0, n) so it will never return
    // parent1.nrows() which would overflow
    let crossover_portion = rng.gen_range(0, parent1.nrows());

    parent1_iter
        .take(crossover_portion)
        .chain(parent2_iter.skip(crossover_portion))
        .collect()
}

fn mutate(individual: &mut Vec<Number>, rng: &mut ThreadRng, config: &PopulationConfig) {
    for gen in individual.iter_mut() {
        *gen = *gen ^ (config.mutation_probability >= rng.gen::<f64>()) as Number;
    }
}
