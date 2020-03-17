use crate::consts::DynResult;
use crate::population::Population;
use crate::population_config::PopulationConfig;
use crate::scenario::Scenario;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn train(
    input_file: String,
    population_config: PopulationConfig,
    generation_limit: usize,
) -> DynResult<()> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let scenario = Scenario::load(input_file)?;

    let tp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("scenario loading {:?}", tp - ts);

    let mut population = Population::new(scenario, population_config);

    let te = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("population generation {:?}", te - tp);

    let mut results = Vec::new();

    for _ in 0..generation_limit {
        let best_individual = population.evolve();
        results.push(best_individual / 1000);
    }

    let tf = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("population evaluation {:?}", tf - te);

    println!("{:?}", results);

    Ok(())
}
