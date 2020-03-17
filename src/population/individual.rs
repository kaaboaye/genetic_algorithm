use crate::consts::Number;
use crate::population::config::Config;
use rand::prelude::ThreadRng;
use rand::Rng;

type Individual<'a> = na::Matrix<
    Number,
    na::U1,
    na::Dynamic,
    na::SliceStorage<'a, Number, na::U1, na::Dynamic, na::U1, na::Dynamic>,
>;

pub fn new_individual(
    child: &mut [Number],
    parent1: &Individual,
    parent2: &Individual,
    rng: &mut ThreadRng,
    config: &Config,
) {
    crossover(child, parent1, parent2, rng, config);
    mutate(child, rng, config);
}

fn crossover(
    child: &mut [Number],
    parent1: &Individual,
    parent2: &Individual,
    rng: &mut ThreadRng,
    config: &Config,
) {
    let parent1_iter = parent1.iter().cloned();
    let parent2_iter = parent2.iter().cloned();

    if config.crossover_probability < rng.gen::<f64>() {
        for (i, n) in parent1_iter.enumerate() {
            child[i] = n;
        }

        return;
    }

    // rng.gen_range generates [0, n) so it will never return
    // parent1.nrows() which would overflow
    let crossover_portion = rng.gen_range(0, parent1.nrows());

    let child_iter = parent1_iter
        .take(crossover_portion)
        .chain(parent2_iter.skip(crossover_portion));

    for (i, n) in child_iter.enumerate() {
        child[i] = n;
    }
}

fn mutate(individual: &mut [Number], rng: &mut ThreadRng, config: &Config) {
    for gen in individual.iter_mut() {
        *gen = *gen ^ (config.mutation_probability >= rng.gen::<f64>()) as Number;
    }
}
