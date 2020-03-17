extern crate crossbeam;
extern crate crossbeam_utils;
extern crate nalgebra as na;
extern crate rand;
extern crate rayon;

mod consts;
mod individual;
mod population;
mod population_config;
mod scenario;
mod train;

use crate::consts::{DynResult, Number};
use crate::population_config::PopulationConfig;
use crate::scenario::Scenario;
use crate::Opt::{Generate, PrintScenario, Train};
use structopt::StructOpt;
use train::train;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Genet",
    about = "
Genetic algorithm implementation solving Knapsack problem.
The main goal of this implementation is to be as fast as it is possible"
)]
enum Opt {
    #[structopt(about = "Generates new scenario and saves it to output file")]
    Generate {
        output_file: String,

        #[structopt(short, long, help = "Output file")]
        number_of_objects: Number,

        #[structopt(short = "w", long, help = "Max weight [Number]")]
        max_weight: Number,

        #[structopt(short = "s", long, help = "Max size [Number]")]
        max_size: Number,
    },

    #[structopt(about = "Generates new population and trains it for given scenario")]
    Train {
        #[structopt(help = "Input file")]
        input_file: String,

        #[structopt(short = "l", long, help = "Generation Limit [usize]")]
        generation_limit: usize,

        #[structopt(short, long, help = "Population Size [usize]")]
        population_size: usize,

        #[structopt(
            short,
            long,
            help = "Tournament Size [usize]. Has to be in range [1, population_size]"
        )]
        tournament_size: usize,

        #[structopt(
            short,
            long,
            help = "Crossover Probability [float64]. Has to be in range [0, 1]"
        )]
        crossover_probability: f64,

        #[structopt(
            short,
            long,
            help = "Mutation Probability [float64]. Has to be in range [0, 1]"
        )]
        mutation_probability: f64,

        #[structopt(
            short,
            long,
            help = "Epsilon [float64]. Stops training when changes between generation are smaller then epsilon"
        )]
        epsilon: f64,
    },

    #[structopt(about = "Loads and prints given scenario")]
    PrintScenario {
        #[structopt(help = "Input file")]
        input_file: String,
    },
}

fn main() -> DynResult<()> {
    match Opt::from_args() {
        Generate {
            number_of_objects,
            max_weight,
            max_size,
            output_file,
        } => Scenario::generate(number_of_objects, max_weight, max_size, output_file)?,

        Train {
            input_file,
            generation_limit,
            population_size,
            tournament_size,
            crossover_probability,
            mutation_probability,
            epsilon,
        } => {
            let population_config = PopulationConfig {
                population_size,
                tournament_size,
                crossover_probability,
                mutation_probability,
            };
            train(input_file, population_config, generation_limit, epsilon)?
        }

        PrintScenario { input_file } => {
            let scenario = Scenario::load(input_file)?;
            println!("{:?}", scenario);
        }
    }

    Ok(())
}
