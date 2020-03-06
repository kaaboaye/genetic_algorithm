use std::fs::File;
use std::io::Write;
use rand::distributions::{Distribution, Uniform};

pub fn generate(number_of_objects: u64, max_weight: u64, max_size: u64, output_file: String)
                -> std::io::Result<()> {
    println!("Generating for max_weight: {}, max_weight: {}, max_size: {}",
             number_of_objects,
             max_weight,
             max_size);


    let mut weight_sum = 0.0;
    let mut size_sum = 0.0;

    {
        let mut file = File::create(&output_file)?;

        // Write header
        file.write_fmt(format_args!("{},{},{}\n", &number_of_objects, &max_weight, &max_size))?;

        let mut rng = rand::thread_rng();

        let weights = Uniform::from(
            0.000001..(10.0 * max_weight as f64 / number_of_objects as f64));

        let sizes = Uniform::from(
            0.000001..(10.0 * max_size as f64 / number_of_objects as f64));

        let costs = Uniform::from(
            0.000001..(number_of_objects as f64));

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
    if weight_sum <= 2.0 * max_weight as f64 || size_sum <= max_size as f64 {
        println!("Validation field");
        return generate(number_of_objects, max_weight, max_size, output_file);
    }

    println!("Generated into: {}", &output_file);
    Ok(())
}
