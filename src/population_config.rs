#[derive(Debug)]
pub struct PopulationConfig {
    pub mutation_probability: f64,
    pub crossover_probability: f64,
    pub crossover_portion: usize,
    pub population_size: usize,
    pub tournament_size: usize,
}
