use std::fs::File;
use std::io::Write;
use rand::distributions::{Distribution, Uniform};

pub fn generate(number_of_objects: u32, max_weight: u32, max_size: u32, output_file: String)
                -> std::io::Result<()> {
    println!("Generating for max_weight: {}, max_weight: {}, max_size: {}",
             number_of_objects,
             max_weight,
             max_size);


    let mut rng = rand::thread_rng();

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
        return generate(number_of_objects, max_weight, max_size, output_file);
    }

    println!("Generated into: {}", &output_file);
    Ok(())
}
