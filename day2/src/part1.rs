#[derive(Debug, Clone, Copy)]
struct Grab {
    red: u32,
    green: u32,
    blue: u32,
}

// Set the values of colors to zero by default because not all grabs have all colors
impl Default for Grab {
    fn default() -> Self {
        Grab {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug)]
struct Game {
    grabs: Vec<Grab>,
}

fn parse_grabs(input: Vec<&str>) -> Vec<Grab> {
    input.iter().map(|grab_str| {
        let mut grab = Grab::default();
        // Split the string into the colors and iterate over them
        for part in grab_str.split(", ") {
            let parts: Vec<&str> = part.split_whitespace().collect();
            if parts.len() == 2 {
                let value = parts[0].parse::<u32>().unwrap_or(0);
                match parts[1] {
                    "red" => grab.red = value,
                    "green" => grab.green = value,
                    "blue" => grab.blue = value,
                    _ => {} // Handle unexpected color
                }
            }
        }
        grab
    }).collect()
}

pub fn solve(input: &str) -> i32 {

    // First parse the input into a usable format

    // Vector to hold the game data
    let mut games = Vec::new();

    // Enumerate over the lines so we can access the index which is also the game number
    for line in input.lines() { 
        // Remove the "Game: " prefix and split the grabs into a vector
        let raw_grabs = line.split(": ").nth(1).unwrap().split("; ").collect::<Vec<_>>();
        // Parse the grabs into a vector of Grab structs
        let grabs = parse_grabs(raw_grabs);
        // Add the game to the games vector
        games.push(Game { grabs });
    }

    // Then loop over the data to find the valid games
    let mut id_sum = 0;

    for (_index, game) in games.iter().enumerate() {
        let (max_red, max_green, max_blue) = game.grabs.iter().fold((0, 0, 0), |(max_red, max_green, max_blue), grab| {
            (
                max_red.max(grab.red),
                max_green.max(grab.green),
                max_blue.max(grab.blue),
            )
        });

        // Rewritten first part solution
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            id_sum += _index+1;
        }
    }
    return id_sum as i32;
}