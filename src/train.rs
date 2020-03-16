use crate::scenario::Scenario;
use crate::consts::DynResult;
use crate::population::Population;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::population_config::PopulationConfig;

pub fn train(input_file: String, population_size: usize) -> DynResult<()> {
    let conf = PopulationConfig {
        population_size,
        tournament_size: 70 * population_size / 100,
        crossover_portion: 3,
        crossover_probability: 0.7,
        mutation_probability: 0.05,
    };

    let ts = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let scenario = Scenario::load(input_file)?;


    let tp = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("scenario loading {:?}", tp - ts);

    let population = Population::new(scenario, conf);


    let te = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("population generation {:?}", te - tp);

//    population.evaluate();

    let tf = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("population evaluation {:?}", tf - te);

    Ok(())
}
