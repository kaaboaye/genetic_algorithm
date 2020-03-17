use crate::consts::DynResult;
use crate::consts::Number;
use crate::population::config::Config as PopulationConfig;
use crate::population::Population;
use crate::scenario::Scenario;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn train(
    input_file: String,
    population_config: PopulationConfig,
    generation_limit: usize,
    epsilon: f64,
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

    let mut previous_best_individual = Number::max_value();

    for _ in 0..generation_limit {
        let best_individual = population.evolve();

        results.push(if best_individual >= 1000 {
            (best_individual / 1000) as f64
        } else {
            best_individual as f64 / 1000.0
        });

        let delta =
            ((best_individual - previous_best_individual) as f64).abs() / best_individual as f64;

        previous_best_individual = best_individual;

        if delta <= epsilon {
            break;
        }
    }

    let tf = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("population evaluation {:?}", tf - te);

    println!("{:?}", results);

    Ok(())
}
