use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
    Unknown
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
    // for line in &grid {
    //     println!("{:?}", line);
    // }

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
    // Look arounnd S to find a starting point
    let mut direction = Direction::Unknown;
    
    // Check if current (position + direction) is within bounds, and if so check if it contains a valid character
    fn set_direction_if_valid(grid: &Vec<Vec<char>>, current_position: &Point, direction: &mut Direction, moves: &HashMap<Direction, Point>, pairings: &HashMap<(Direction, char), Direction>, new_direction: Direction) {
        if *direction == Direction::Unknown && is_within_bounds(&grid, &current_position.add_move(&moves[&new_direction])) {
            if let Some(pairing_direction) = pairings.get(&(new_direction, get_char_at(&grid, &current_position.add_move(&moves[&new_direction])))){
                *direction = new_direction;
            }
        }
    }
    set_direction_if_valid(&grid, &current_position, &mut direction, &moves, &pairings, Direction::North);
    set_direction_if_valid(&grid, &current_position, &mut direction, &moves, &pairings, Direction::East);
    set_direction_if_valid(&grid, &current_position, &mut direction, &moves, &pairings, Direction::South);
    set_direction_if_valid(&grid, &current_position, &mut direction, &moves, &pairings, Direction::West);

    //println!("Starting position: {:?}, direction: {:?}", current_position, direction);
    
    let mut steps = 0;
    loop {
        steps += 1;
        //println!("Steps: {}, Position: {:?}, Direction: {:?}", steps, current_position, direction);
        current_position = current_position.add_move(&moves[&direction]);
        let c = get_char_at(&grid, &current_position);
        if c == 'S' {
            break;
        }
        if let Some(new_direction) = pairings.get(&(direction, c)) {
            direction = *new_direction;
        }
    }
    return steps / 2 as i64;
}
