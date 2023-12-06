#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_input(input: &str) -> Race {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<String> = lines[0]
        .split_whitespace()
        .skip(1) // skipping "Time:"
        .map(|s| s.to_string())
        .collect();
    let distances: Vec<String> = lines[1]
        .split_whitespace()
        .skip(1) // skipping "Distance:"
        .map(|s| s.to_string())
        .collect();

    let time_str = times.join("");
    let distance_str = distances.join("");

    let time = time_str.parse::<u64>().unwrap_or(0);
    let distance = distance_str.parse::<u64>().unwrap_or(0);

    println!("Times: {:?}", times);
    println!("Distances: {:?}", distances);

    println!("Time: {:?}", time);
    println!("Distance: {:?}", distance);


    Race { time, distance }
}


#[allow(unused_variables)]
pub fn solve(input: &str) -> i32 {
    let race = parse_input(input);

    
    println!("Race: {:?}", race);
    // Create list of possible options
    let options: Vec<u64> = (1..=race.time).collect();
    let mut valid_options: Vec<u64> = Vec::new();

    for option in options.iter() {
        if (race.time - option) * option > race.distance {
            valid_options.push(*option);
        }
    }

    let all_options = valid_options.len();
    return all_options as i32;
}
