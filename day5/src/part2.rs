#[derive(Debug)]
struct Map {
    destination_start: i64,
    destination_end: i64,
    source_start: i64,
    source_end: i64,
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
    let mut seeds = seeds_line.split(": ")
        .nth(1).unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
    // Make every 2nd seed number the sum of the previous 2 seed numbers for easier calculations
    let mut i = 1;
    while i < seeds.len() {
        seeds[i] = seeds[i-1] + seeds[i];
        i+=2;
    }
    
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
            // Save the source_end instead of the range to make calculations easier
            let range = parts[2].parse::<i64>().unwrap();
            let source_start = parts[1].parse::<i64>().unwrap() - 1;
            let source_end = source_start + range;
            let destination_start = parts[0].parse::<i64>().unwrap();
            let destination_end = destination_start + range - 1;
            let map = Map {
                destination_start: parts[0].parse().unwrap(),
                destination_end: destination_end,
                source_start: parts[1].parse().unwrap(),
                source_end: source_end
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
            println!("Map: {} {} <= {} {}", map.destination_start, map.destination_end, map.source_start, map.source_end);
        }
    }
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i32 {
    let data = parse_input(input);
     //print_data(&data);
    
    // Create seed ranges vector of undefined size
    let mut seeds: Vec<i64> = Vec::new();
    let mut state: Vec<i64> = Vec::new();

    for (i, seed) in data.seeds.iter().enumerate() {
        seeds.push(*seed);
    }
    
    // Loop through the converters
    let print_j = 4;
    println!("Starting state         : {:?}", seeds);

    // Loop through the converters
    for (j,converter) in data.converters.iter().enumerate() {
        let mut i = 0;
        while i < seeds.len() {
            let [seed_start, seed_end] = [seeds[i] as i64, seeds[i+1] as i64];
            let mut found_map = false;
            for map in &converter.maps {
                let [source_start, source_end] = [map.source_start as i64, map.source_end as i64];
                let [destination_start, destination_end] = [map.destination_start as i64, map.destination_end as i64];
                // Check if the seed_start is in the map
                if seed_start >= source_start && seed_start <= source_end {
                    // The start of the seed range is in the map. Check if the end also fits in the map.
                    if seed_end <= source_end {
                        // The end of the seed range also fits in the map. Add the difference to the destination start.
                        let result_start = destination_start + (seed_start - source_start);
                        let result_end = result_start + (seed_end - seed_start);
                        state.push(result_start);
                        state.push(result_end);
                        println!("Start and end fit. Seed: {} {}. Source: {} {}. Destination: {} {}. Result: {} {}", seed_start, seed_end, source_start, source_end, destination_start, destination_end, result_start, result_end);
                        
                    } else {
                        // The end of the seed range does not fit in the map. Add the fitting part to the state vector, and the missing part to the seed_ranges vector.
                        let result_start = destination_start + (seed_start - source_start);
                        let result_end = destination_end;
                        state.push(result_start);
                        state.push(result_end);
                        
                        let missing_start = source_end + 1;
                        let missing_end = seed_end;
                        seeds.push(missing_start);
                        seeds.push(missing_end);
                        
                        
                        println!("Start fits, end does not. Seed: {} {}. Source: {} {}. Destination: {} {}. Result: {} {}. Missing: {} {}", seed_start, seed_end, source_start, source_end, destination_start, destination_end, result_start, result_end, missing_start, missing_end);
                        
                    }
                    found_map = true;
                    break;
                } else if seed_end >= source_start && seed_end <= source_end {
                    // The end of the seed range fits in the map. Add the fitting part to the state vector, and the missing part to the seed_ranges vector.
                    let result_start = destination_start;
                    let result_end = destination_start + (seed_end - source_start);
                    state.push(result_start);
                    state.push(result_end);
                    
                    
                    let missing_start = seed_start;
                    let missing_end = source_start-1;
                    seeds.push(missing_start);
                    seeds.push(missing_end);
                    println!("End fits, start does not. Seed: {} {}. Source: {} {}. Destination: {} {}. Result: {} {}. Missing: {} {}", seed_start, seed_end, source_start, source_end, destination_start, destination_end, result_start, result_end, missing_start, missing_end);
                    found_map = true;
                    break;
                }
            }
            // If no map is found, keep the seed number
            if !found_map {
                state.push(seed_start);
                state.push(seed_end);

                println!("No map found. Seed: {} {}. Missing: {} {}", seed_start, seed_end, seed_start, seed_end);
            }
            i+=2;
        }

        // Overwrite seeds with state
        seeds = state.clone();
        state.clear();
        println!("Seed ranges after converter {}: {:?}", j, seeds);
    }

    println!("\n Final State: {:?}", state); 
    
    // Return smallest number in state as i32
    return *seeds.iter().min().unwrap() as i32;
}