extern crate rand;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate crossbeam;
extern crate crossbeam_utils;

mod consts;
mod population;
mod scenario;
mod train;

use structopt::StructOpt;
use train::train;
use crate::consts::{Number, DynResult};
use crate::Opt::{Generate, PrintScenario, Train};
use crate::scenario::Scenario;

#[derive(Debug, StructOpt)]
#[structopt(name = "Genet", about = "Genetic algorithm")]
enum Opt {
    Generate {
        number_of_objects: Number,
        max_weight: Number,
        max_size: Number,
        output_file: String,
    },

    Train {
        input_file: String,
        population_size: usize,
    },

    PrintScenario {
        input_file: String
    },
}

fn main() -> DynResult<()> {
    match Opt::from_args() {
        Generate {
            number_of_objects, max_weight, max_size, output_file
        } => Scenario::generate(number_of_objects, max_weight, max_size, output_file)?,

        Train { input_file, population_size } => train(input_file, population_size)?,

        PrintScenario { input_file } => {
            let scenario = Scenario::load(input_file)?;
            println!("{:?}", scenario);
        }
    }

    Ok(())
}
