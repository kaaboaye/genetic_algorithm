#[derive(Debug)]
pub struct Config {
    pub mutation_probability: f64,
    pub crossover_probability: f64,
    pub population_size: usize,
    pub tournament_size: usize,
}
