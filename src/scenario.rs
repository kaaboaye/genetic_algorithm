use na::DVector;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use crate::consts::{Number, DynResult};
use std::error::Error;
use std::result::Result;
use std::fmt::{Display, Formatter};
use std::{fmt, io};
use rand::distributions::{Uniform, Distribution};
use rand::thread_rng;
use crate::scenario::StrategyLoadError::{NoHeader, HeaderLengthOtherThen3, LineLengthOtherThen3, IncorrectNumberOfObjects, TotalWeightToSmall, TotalSizeToSmall};

#[derive(Debug)]
pub struct Scenario {
    pub weights: DVector<Number>,
    pub sizes: DVector<Number>,
    pub costs: DVector<Number>,
    pub number_of_objects: Number,
    pub max_weight: Number,
    pub max_size: Number,
}

impl Scenario {
    pub fn generate(number_of_objects: Number, max_weight: Number, max_size: Number, output_file: String)
                    -> io::Result<()> {
        println!("Generating for max_weight: {}, max_weight: {}, max_size: {}",
                 number_of_objects,
                 max_weight,
                 max_size);


        let mut rng = thread_rng();

        let weights = Uniform::from(1..(10 * max_weight / number_of_objects));
        let sizes = Uniform::from(1..(10 * max_size / number_of_objects));
        let costs = Uniform::from(1..number_of_objects);

        let mut weight_sum = 0;
        let mut size_sum = 0;

        {
            let mut file = File::create(&output_file)?;

            // Write header
            file.write_fmt(format_args!("{},{},{}\n", &number_of_objects, &max_weight, &max_size))?;


            for _ in 0..number_of_objects {
                let weight = weights.sample(&mut rng);
                let size = sizes.sample(&mut rng);
                let cost = costs.sample(&mut rng);

                weight_sum += weight;
                size_sum += size;

                file.write_fmt(format_args!("{},{},{}\n", weight, size, cost))?;
            }
        }

        // Validate the results
        println!("Weight sum: {} Size sum: {}", weight_sum, size_sum);
        if weight_sum <= 2 * max_weight || size_sum <= 2 * max_size {
            println!("Validation field");
            return Scenario::generate(number_of_objects, max_weight, max_size, output_file);
        }

        println!("Generated into: {}", &output_file);
        Ok(())
    }

    pub fn load(input_file: String) -> DynResult<Scenario> {
        let file = File::open(input_file)?;
        let mut lines = BufReader::new(file)
            .lines()
            .map(|line| -> DynResult<Vec<Number>> {
                let parse_line = line?
                    .split(",")
                    .map(|str_num| str_num.parse::<Number>())
                    .collect::<Result<Vec<Number>, _>>()?;

                Ok(parse_line)
            });

        let header = lines.next().ok_or(NoHeader)??;
        if header.len() != 3 { Err(HeaderLengthOtherThen3)? }

        let (number_of_objects, max_weight, max_size) = (header[0], header[1], header[2]);

        let mut weights = Vec::new();
        let mut sizes = Vec::new();
        let mut costs = Vec::new();

        for line in lines {
            let line = line?;
            if line.len() != 3 { Err(LineLengthOtherThen3)? }

            weights.push(line[0]);
            sizes.push(line[1]);
            costs.push(line[2]);
        }

        if weights.len() != number_of_objects as usize {
            Err(IncorrectNumberOfObjects { declared: number_of_objects, actual: weights.len() })?
        }

        let total_weight = weights.iter().sum();
        if total_weight <= 2 * max_weight {
            Err(TotalWeightToSmall { minimal: 2 * max_weight, total_weight })?
        }

        let total_size = sizes.iter().sum();
        if total_size <= 2 * max_size {
            Err(TotalSizeToSmall { minimal: 2 * max_size, total_size })?
        }

        let weights = DVector::from_vec(weights);
        let sizes = DVector::from_vec(sizes);
        let costs = DVector::from_vec(costs);

        Ok(Scenario { number_of_objects, max_weight, max_size, weights, sizes, costs })
    }
}


#[derive(Debug)]
pub enum StrategyLoadError {
    NoHeader,
    HeaderLengthOtherThen3,
    LineLengthOtherThen3,

    IncorrectNumberOfObjects {
        declared: Number,
        actual: usize,
    },

    TotalWeightToSmall {
        minimal: Number,
        total_weight: Number,
    },

    TotalSizeToSmall {
        minimal: Number,
        total_size: Number,
    },
}

impl Error for StrategyLoadError {}

impl Display for StrategyLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
