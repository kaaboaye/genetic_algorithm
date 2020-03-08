use crate::scenario::Scenario;
use crate::consts::DynResult;
use crate::population::Population;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn train(input_file: String, population_size: usize) -> DynResult<()> {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let scenario = Scenario::load(input_file)?;


    let tp = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("scenario loading {:?}", tp - ts);

    let population = Population::new(population_size, scenario.number_of_objects as usize);


    let te = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("population generation {:?}", te - tp);

    population.evaluate(&scenario);

    let tf = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("population evaluation {:?}", tf - te);

    Ok(())
}
