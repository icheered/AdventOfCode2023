#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Object {
    Empty,
    Vertical,
    Horizontal,
    Slash,
    Backslash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}

fn parse_input(input: &str) -> Vec<Vec<Object>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(Object::Empty),
                '|' => row.push(Object::Vertical),
                '-' => row.push(Object::Horizontal),
                '/' => row.push(Object::Slash),
                '\\' => row.push(Object::Backslash),
                _ => panic!("Invalid input"),
            }
        }
        grid.push(row);
    }
    grid
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Object>>) {
    for row in grid {
        for c in row {
            match c {
                Object::Empty => print!("."),
                Object::Vertical => print!("|"),
                Object::Horizontal => print!("-"),
                Object::Slash => print!("/"),
                Object::Backslash => print!("\\"),
            }
        }
        println!();
    }
}

fn get_position(
    current_position: &Position,
    direction: Direction,
    grid_width: usize,
    grid_height: usize,
) -> Option<Position> {
    match direction {
        Direction::Left => {
            if current_position.x > 0 {
                Some(Position {
                    x: current_position.x - 1,
                    y: current_position.y,
                    direction,
                })
            } else {
                None
            }
        }
        Direction::Right => {
            if current_position.x < grid_width - 1 {
                Some(Position {
                    x: current_position.x + 1,
                    y: current_position.y,
                    direction,
                })
            } else {
                None
            }
        }
        Direction::Up => {
            if current_position.y > 0 {
                Some(Position {
                    x: current_position.x,
                    y: current_position.y - 1,
                    direction,
                })
            } else {
                None
            }
        }
        Direction::Down => {
            if current_position.y < grid_height - 1 {
                Some(Position {
                    x: current_position.x,
                    y: current_position.y + 1,
                    direction,
                })
            } else {
                None
            }
        }
    }
}

// Function that takes a position and returns the next position(s)
fn step(grid: &Vec<Vec<Object>>, position: Position) -> Vec<Position> {
    let mut positions = Vec::new();

    match position.direction {
        Direction::Up => match grid[position.y][position.x] {
            Object::Empty | Object::Vertical => {
                if let Some(new_position) =
                    get_position(&position, Direction::Up, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Horizontal => {
                if let Some(new_position) =
                    get_position(&position, Direction::Left, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
                if let Some(new_position) =
                    get_position(&position, Direction::Right, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Slash => {
                if let Some(new_position) =
                    get_position(&position, Direction::Right, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Backslash => {
                if let Some(new_position) =
                    get_position(&position, Direction::Left, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
        },
        Direction::Down => match grid[position.y][position.x] {
            Object::Empty | Object::Vertical => {
                if let Some(new_position) =
                    get_position(&position, Direction::Down, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Horizontal => {
                if let Some(new_position) =
                    get_position(&position, Direction::Left, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
                if let Some(new_position) =
                    get_position(&position, Direction::Right, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Slash => {
                if let Some(new_position) =
                    get_position(&position, Direction::Left, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Backslash => {
                if let Some(new_position) =
                    get_position(&position, Direction::Right, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
        },
        Direction::Left => match grid[position.y][position.x] {
            Object::Empty | Object::Horizontal => {
                if let Some(new_position) =
                    get_position(&position, Direction::Left, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Vertical => {
                if let Some(new_position) =
                    get_position(&position, Direction::Up, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
                if let Some(new_position) =
                    get_position(&position, Direction::Down, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Slash => {
                if let Some(new_position) =
                    get_position(&position, Direction::Down, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Backslash => {
                if let Some(new_position) =
                    get_position(&position, Direction::Up, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
        },
        Direction::Right => match grid[position.y][position.x] {
            Object::Empty | Object::Horizontal => {
                if let Some(new_position) =
                    get_position(&position, Direction::Right, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Vertical => {
                if let Some(new_position) =
                    get_position(&position, Direction::Up, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
                if let Some(new_position) =
                    get_position(&position, Direction::Down, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Slash => {
                if let Some(new_position) =
                    get_position(&position, Direction::Up, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
            Object::Backslash => {
                if let Some(new_position) =
                    get_position(&position, Direction::Down, grid[0].len(), grid.len())
                {
                    positions.push(new_position);
                }
            }
        },
    }
    positions
}

fn update_visited_grid(
    grid: &mut [[bool; 200]; 200],
    visited_positions: &mut [[[bool; 4]; 200]; 200],
    positions: &Vec<Position>,
) {
    for position in positions {
        grid[position.y][position.x] = true;
        visited_positions[position.y][position.x][position.direction as usize] = true;
    }
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let grid = parse_input(input);
    //print_grid(&grid);

    // Create a list of all positions where x=0 and direction is right
    let right_directions = (0..grid.len())
        .map(|y| Position {
            x: 0,
            y,
            direction: Direction::Right,
        })
        .collect::<Vec<Position>>();

    let down_directions = (0..grid[0].len())
        .map(|x| Position {
            x,
            y: 0,
            direction: Direction::Down,
        })
        .collect::<Vec<Position>>();

    let left_directions = (0..grid.len())
        .map(|y| Position {
            x: grid[0].len() - 1,
            y,
            direction: Direction::Left,
        })
        .collect::<Vec<Position>>();

    let up_directions = (0..grid[0].len())
        .map(|x| Position {
            x,
            y: grid.len() - 1,
            direction: Direction::Up,
        })
        .collect::<Vec<Position>>();

    let mut start_positions = Vec::new();
    start_positions.extend(right_directions);
    start_positions.extend(down_directions);
    start_positions.extend(left_directions);
    start_positions.extend(up_directions);

    let mut best_visited_tiles = 0;

    //println!("Start positions: {:?}", start_positions);

    for position in &start_positions {
        // Visited cells
        let mut visited = [[false; 200]; 200];

        // Visited positions to keep track if we are in a loop
        let mut visited_positions = [[[false; 4]; 200]; 200];

        // Start with the first position
        let mut positions = Vec::new();
        positions.push(*position); // Dereference and clone the position

        loop {
            update_visited_grid(&mut visited, &mut visited_positions, &positions);
            let mut new_positions = Vec::new();
            for position in positions {
                let next_positions = step(&grid, position);
                new_positions.extend(next_positions);
            }
            // Remove positions with direction that have already been visited visited_positions
            new_positions = new_positions
                .into_iter()
                .filter(|position| {
                    !visited_positions[position.y][position.x][position.direction as usize]
                })
                .collect();

            //println!("New positions: {:?}", new_positions);
            if new_positions.is_empty() {
                break;
            }
            positions = new_positions;
        }

        let visited_tiles = visited
            .iter()
            .flatten()
            .filter(|&&x| x)
            .collect::<Vec<&bool>>()
            .len();
        if visited_tiles > best_visited_tiles {
            best_visited_tiles = visited_tiles;
            // println!("Best visited tiles: {}", best_visited_tiles);
            // println!("Start position: {:?}", position);
        }
    }

    best_visited_tiles as i64
}
