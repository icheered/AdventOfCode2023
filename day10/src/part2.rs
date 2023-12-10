use std::collections::HashMap;
use colored::*;


#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
    Unknown
}

impl Direction {
    fn to_index(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::Unknown => 4,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn add_move(&self, direction: &Point) -> Point {
        Point {
            x: self.x + direction.x,
            y: self.y + direction.y,
        }
    }
}


fn get_direction_pairings() -> HashMap<(Direction, char), Direction> {
    let mut pairings = HashMap::new();

    pairings.insert((Direction::North, '|'), Direction::North);
    pairings.insert((Direction::North, 'F'), Direction::East);
    pairings.insert((Direction::North, '7'), Direction::West);

    pairings.insert((Direction::East, '-'), Direction::East);
    pairings.insert((Direction::East, '7'), Direction::South);
    pairings.insert((Direction::East, 'J'), Direction::North);

    pairings.insert((Direction::South, '|'), Direction::South);
    pairings.insert((Direction::South, 'L'), Direction::East);
    pairings.insert((Direction::South, 'J'), Direction::West);

    pairings.insert((Direction::West, '-'), Direction::West);
    pairings.insert((Direction::West, 'L'), Direction::North);
    pairings.insert((Direction::West, 'F'), Direction::South);
    pairings
}

fn get_direction_moves() -> HashMap<Direction, Point> {
    let mut moves = HashMap::new();

    moves.insert(Direction::North, Point { x: 0, y: -1 });
    moves.insert(Direction::East, Point { x: 1, y: 0 });
    moves.insert(Direction::South, Point { x: 0, y: 1 });
    moves.insert(Direction::West, Point { x: -1, y: 0 });

    moves
}


// Make a grid with the input, every character its own cell
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn is_within_bounds(grid: &Vec<Vec<char>>, point: &Point) -> bool {
    // First, check if the point's x and y coordinates are non-negative.
    if point.x < 0 || point.y < 0 {
        return false;
    }

    // Convert the coordinates to usize for indexing.
    let x = point.x as usize;
    let y = point.y as usize;

    // Check if the x coordinate is within the width of the grid
    // and the y coordinate is within the height of the grid.
    y < grid.len() && x < grid[0].len()
}

fn get_char_at(grid: &Vec<Vec<char>>, point: &Point) -> char {
    grid[point.y as usize][point.x as usize]
}



#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let grid = parse_input(input);

    // Create a hashmap with all the possible pairings of directions and characters
    let pairings = get_direction_pairings();
    let moves = get_direction_moves();

    // Find the starting point, the letter S on the grid
    let mut current_position = Point { x: 0, y: 0 };
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == 'S' {
                current_position = Point { x: x.try_into().unwrap(), y: y.try_into().unwrap() };
            }
        }
    }
    
    // Look arounnd S to find a starting direction
    let mut direction = Direction::Unknown;
    let mut valid_starting_directions = [false, false, false, false];
    
    // Check if current (position + direction) is within bounds, and if so check if it contains a valid character
    fn set_direction_if_valid(grid: &Vec<Vec<char>>, current_position: &Point, direction: &mut Direction, moves: &HashMap<Direction, Point>, pairings: &HashMap<(Direction, char), Direction>, valid_starting_directions: &mut [bool; 4], new_direction: Direction) {
        if is_within_bounds(&grid, &current_position.add_move(&moves[&new_direction])) {
            if let Some(pairing_direction) = pairings.get(&(new_direction, get_char_at(&grid, &current_position.add_move(&moves[&new_direction])))){
                *direction = new_direction;
                valid_starting_directions[new_direction.to_index()] = true;
            }
        }
    }
    set_direction_if_valid(&grid, &current_position, &mut direction, &moves, &pairings, &mut valid_starting_directions, Direction::North);
    set_direction_if_valid(&grid, &current_position, &mut direction, &moves, &pairings, &mut valid_starting_directions, Direction::East);
    set_direction_if_valid(&grid, &current_position, &mut direction, &moves, &pairings, &mut valid_starting_directions, Direction::South);
    set_direction_if_valid(&grid, &current_position, &mut direction, &moves, &pairings, &mut valid_starting_directions, Direction::West);

    //println!("valid_starting_directions: {:?}", valid_starting_directions);

    // Create an empty grid to fill in with the path
    let mut path = grid.clone();
    for line in &mut path {
        for c in line {
            *c = '.';
        }
    }
    path[current_position.y as usize][current_position.x as usize] = 'S';

    // Fill in the path
    loop {
        current_position = current_position.add_move(&moves[&direction]);
        let c = get_char_at(&grid, &current_position);
        if c == 'S' {
            break;
        }
        if let Some(new_direction) = pairings.get(&(direction, c)) {
            direction = *new_direction;
        }
        path[current_position.y as usize][current_position.x as usize] = c;
    }

    // Replase the S with the correct character depending on which neighbouring squares have a path
    path[current_position.y as usize][current_position.x as usize] = match valid_starting_directions {
        [true, true, false, false] => 'L',
        [true, false, true, false] => '|',
        [true, false, false, true] => 'J',
        [false, true, true, false] => 'F',
        [false, true, false, true] => '-',
        [false, false, true, true] => '7',
        _ => panic!("Invalid starting position"),
    };

    // Copy the grid with same size as path but with boolean values all False
    //let mut in_path = vec![vec![false; path[0].len()]; path.len()];

    // For every value in the boolean grid count the number of intersections
    let intersections = ["|", "F", "7"];

    let mut count = 0;

    path.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &c)| {
            // Check if position is an empty space
            if c != '.' {
                return;
            }

            let mut intersection_count = 0;
            // Count number of intersections to the RIGHT of this position
            for x2 in x+1..line.len() {
                if intersections.contains(&path[y][x2].to_string().as_str()) {
                    intersection_count += 1;
                }
            }

            // If number of intersections is odd, change it to true
            if intersection_count % 2 == 1 {
                //in_path[y][x] = true;
                count += 1;
            }
        });
    });
    
    // // If the number is odd, change it to true
    // for (i, line) in path.iter().enumerate() {
    //     print!("{:02} ", i);
    //     for (j, &c) in line.iter().enumerate() {
    //         let is_inside = in_path[i][j];
    //         if is_inside {
    //             print!("{}", c.to_string().green());
    //         } else {
    //             print!("{}", c.to_string().red());
    //         }
    //     }
    //     println!(); // Newline after each row
    // }
    
    return count as i64;
}
