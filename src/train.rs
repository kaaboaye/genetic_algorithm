use crate::scenario::Scenario;
use crate::consts::DynResult;
use crate::population::Population;

pub fn train(input_file: String, population_size: usize) -> DynResult<()> {
    let scenario = Scenario::load(input_file)?;
    let population = Population::new(population_size, scenario.number_of_objects as usize);

    println!("{:?} {:?}", scenario, population);

    Ok(())
}
