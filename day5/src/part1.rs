#[derive(Debug)]
struct Map {
    destination_start: i64,
    source_start: i64,
    range: i64,
}

#[derive(Debug)]
struct Converter {
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Data {
    seeds: Vec<i64>,
    converters: Vec<Converter>,
}

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();
    
    // Parse seeds
    let seeds_line = lines.next().unwrap();
    let seeds = seeds_line.split(": ")
        .nth(1).unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
    // 

    // Parse converters
    let mut converters = Vec::new();
    let mut maps = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        if line.ends_with("map:") {
            if !maps.is_empty() {
                converters.push(Converter { maps });
                maps = Vec::new();
            }
        } else {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let map = Map {
                destination_start: parts[0].parse().unwrap(),
                source_start: parts[1].parse().unwrap(),
                range: parts[2].parse().unwrap(),
            };
            maps.push(map);
        }
    }

    if !maps.is_empty() {
        converters.push(Converter { maps });
    }

    Data { seeds, converters }
}

#[allow(dead_code)]
fn print_data(data: &Data) {
    println!("Seeds: {:?}", data.seeds);
    for maps in &data.converters {
        println!("---");
        for map in &maps.maps {
            println!("Map: {} {} {}", map.destination_start, map.source_start, map.range);
        }
    }
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i32 {
    let data = parse_input(input);
    //print_data(&data);

    // Create a state vector to hold the seeds
    
    
    // State vector with length of seeds
    let mut previous_state = vec![0; data.seeds.len()];
    let mut state = vec![0; data.seeds.len()];
    for (i, seed) in data.seeds.iter().enumerate() {
        previous_state[i] = *seed;
    }
    
    // Loop through the converters
    println!("Starting state         : {:?}", previous_state);
    for (j,converter) in data.converters.iter().enumerate() {
        // Loop through the seeds
        for (i, seed) in previous_state.iter().enumerate() {
            // Loop through the maps
            let mut found_map = false;
            for map in &converter.maps {
                // If seed number is within [source, source + range), add (destination+difference) to state vector
                if *seed >= map.source_start && *seed < map.source_start + map.range {
                    state[i] = map.destination_start + (*seed - map.source_start);
                    found_map = true;
                    break;
                }
            }
            // If no map is found, keep the seed number
            if !found_map {
                state[i] = *seed;
            }
        }
        // Set previous state to state
        previous_state = state.clone();
        println!("State after converter {}: {:?}", j, state);
    }

    println!("\n Final State: {:?}", state); 
    
    // Return smallest number in state as i32
    return *state.iter().min().unwrap() as i32;
}
