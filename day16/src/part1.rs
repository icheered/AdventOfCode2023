use std::io::{self, Write};
use std::{thread, time};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Object {
    Empty,
    Vertical,
    Horizontal,
    Slash,
    Backslash,
    Right,
    Left,
    Up,
    Down,
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
            // For other cases, do nothing
            _ => {}
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
            // For other cases, do nothing
            _ => {}
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
            // For other cases, do nothing
            _ => {}
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
            // For other cases, do nothing
            _ => {}
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

fn print_grid(grid: &Vec<Vec<Object>>) {
    for row in grid {
        for c in row {
            match c {
                Object::Empty => print!("\x1B[1m\x1B[38;5;2m.\x1B[0m"), // Bold, dark gray for Empty
                Object::Vertical => print!("\x1B[1m\x1B[38;5;7m|\x1B[0m"), // Bold, white for Vertical
                Object::Horizontal => print!("\x1B[1m\x1B[38;5;7m-\x1B[0m"), // Bold, white for Horizontal
                Object::Slash => print!("\x1B[1m\x1B[38;5;7m/\x1B[0m"), // Bold, white for Slash
                Object::Backslash => print!("\x1B[1m\x1B[38;5;7m\\\x1B[0m"), // Bold, white for Backslash
                Object::Right => print!("\x1B[1m\x1B[38;5;1m>\x1B[0m"),      // Bold, red for Right
                Object::Left => print!("\x1B[1m\x1B[38;5;1m<\x1B[0m"),       // Bold, red for Left
                Object::Up => print!("\x1B[1m\x1B[38;5;1m^\x1B[0m"),         // Bold, red for Up
                Object::Down => print!("\x1B[1m\x1B[38;5;1mv\x1B[0m"),       // Bold, red for Down
            }
        }
        println!(); // Move to the next line after printing each row
    }
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let grid = parse_input(input);
    let mut visual_grid = grid.clone();
    io::stdout().flush().unwrap();

    let start_row = 2; // assuming grid starts from the first row of the terminal
    let start_col = 1; // assuming grid starts from the first column of the terminal

    // Visited cells
    let mut visited = [[false; 200]; 200];

    // Visited positions to keep track if we are in a loop
    let mut visited_positions = [[[false; 4]; 200]; 200];

    // Start at 0,0 moving right
    let position = Position {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };

    let mut positions = Vec::new();
    positions.push(position);

    loop {
        update_visited_grid(&mut visited, &mut visited_positions, &positions);
        let mut update = false;
        for position in &positions {
            if grid[position.y][position.x] == Object::Empty {
                update = true;
                match position.direction {
                    Direction::Up => {
                        visual_grid[position.y][position.x] = Object::Up;
                    }
                    Direction::Down => {
                        visual_grid[position.y][position.x] = Object::Down;
                    }
                    Direction::Left => {
                        visual_grid[position.y][position.x] = Object::Left;
                    }
                    Direction::Right => {
                        visual_grid[position.y][position.x] = Object::Right;
                    }
                }
            }
        }
        if update {
            thread::sleep(time::Duration::from_millis(10));
        }
        print!("\x1B[{};{}H", start_row, start_col);
        print_grid(&visual_grid);
        io::stdout().flush().unwrap();

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

    // Calculate number of visited squares
    visited
        .iter()
        .flatten()
        .filter(|&&x| x)
        .collect::<Vec<&bool>>()
        .len() as i64
}
