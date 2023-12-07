use color_eyre::Report;
use std::{env, fs, time::Instant};

// Importing the part1 and part2 modules
mod part1;
mod part2;

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: ./{} <part1|part2> <test|input>", args[0].split("/").last().unwrap());
        std::process::exit(1);
    }
    let part = &args[1];

    // Concatenate ".txt" to the filename provided in the command line argument
    let filename = format!("{}.txt", args[2]);

    // Read the content of the file
    let input = fs::read_to_string(filename)?;

    let start = Instant::now();
    let result = match part.as_str() {
        "part1" => part1::solve(&input),
        "part2" => part2::solve(&input),
        _ => {
            eprintln!("Invalid argument for part: use 'part1' or 'part2'");
            std::process::exit(1);
        }
    };

    println!("\nResult: {}", result);

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration.as_micros());

    Ok(())
}