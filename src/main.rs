extern crate rand;

mod generate;

use structopt::StructOpt;
use generate::generate;

#[derive(Debug, StructOpt)]
#[structopt(name = "Genet", about = "Genetic algorithm")]
enum Opt {
    Generate {
        number_of_objects: u32,
        max_weight: u32,
        max_size: u32,
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
