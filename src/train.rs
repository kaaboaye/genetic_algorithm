use crate::consts::DynResult;
use crate::consts::Number;
use crate::population::config::Config as PopulationConfig;
use crate::population::Population;
use crate::scenario::Scenario;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn train(
    input_file: String,
    result_file: Option<String>,
    population_config: PopulationConfig,
    generation_limit: usize,
    epsilon: Option<f64>,
) -> DynResult<()> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let scenario = Scenario::load(input_file)?;

    let tp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("Scenario loading {:?}", tp - ts);

    println!("{:?}", &population_config);

    let mut population = Population::new(scenario, population_config);

    let te = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("Population generation {:?}", te - tp);

    let mut results = Vec::new();

    let mut previous_best_individual = Number::max_value();

    for _ in 0..generation_limit {
        let best_individual = population.evolve();

        results.push(best_individual);

        let delta =
            ((best_individual - previous_best_individual) as f64).abs() / best_individual as f64;

        previous_best_individual = best_individual;

        if let Some(epsilon) = epsilon {
            if delta <= epsilon {
                break;
            }
        }
    }

    let tf = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("Population evolution {:?}", tf - te);

    if let Some(result_file) = result_file {
        let results = results
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let mut file = File::create(&result_file)?;
        file.write(results.as_bytes())?;
    } else {
        println!("{:?}", results);
    }

    println!("Result {}", results.last().unwrap());

    Ok(())
}
