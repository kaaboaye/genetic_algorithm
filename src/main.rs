extern crate rand;

mod generate;

use structopt::StructOpt;
use generate::generate;

#[derive(Debug, StructOpt)]
#[structopt(name = "Genet", about = "Genetic algorithm")]
enum Opt {
    Generate {
        number_of_objects: u64,
        max_weight: u64,
        max_size: u64,
        output_file: String,
    }
}

fn main() -> std::io::Result<()> {
    match Opt::from_args() {
        Opt::Generate
        {
            number_of_objects, max_weight, max_size, output_file
        } =>
            generate(number_of_objects, max_weight, max_size, output_file)?
    }

    Ok(())
}
