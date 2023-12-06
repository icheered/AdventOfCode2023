#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn parse_input(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<i32> = lines[0]
        .split_whitespace()
        .skip(1) // skipping "Time:"
        .filter_map(|s| s.parse().ok())
        .collect();
    let distances: Vec<i32> = lines[1]
        .split_whitespace()
        .skip(1) // skipping "Distance:"
        .filter_map(|s| s.parse().ok())
        .collect();

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect()
}


#[allow(unused_variables)]
pub fn solve(input: &str) -> i32 {
    let races = parse_input(input);

    let mut valid_options_per_race: Vec<usize> = Vec::new();
    // Loop through races
    for race in races.iter() {
        println!("Race: {:?}", race);
        // Create list of possible options
        let options: Vec<i32> = (1..=race.time).collect();
        let mut valid_options: Vec<i32> = Vec::new();

        for option in options.iter() {
            if (race.time - option) * option > race.distance {
                valid_options.push(*option);
            }
        }
        // Push length of valid_options to valid_options_per_race
        valid_options_per_race.push(valid_options.len());
    }

    // Multiply all values of valid_options_per_race
    let result = valid_options_per_race.iter().fold(1, |acc, x| acc * x);
    return result as i32;
}
