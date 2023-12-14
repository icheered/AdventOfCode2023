// use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
enum Object {
    Circle,
    Square,
    Empty,
}
#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Vec<Object>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(Object::Empty),
                'O' => row.push(Object::Circle),
                '#' => row.push(Object::Square),
                _ => panic!("Invalid input"),
            }
        }
        grid.push(row);
    }
    grid
}

fn move_circles(grid: &mut Vec<Vec<Object>>, direction: Direction) {
    match direction {
        Direction::Up => {
            for j in 0..grid[0].len() {
                let mut empty_row = 0;
                for i in 0..grid.len() {
                    if grid[i][j] == Object::Circle {
                        grid[i][j] = Object::Empty;
                        grid[empty_row][j] = Object::Circle;
                        empty_row += 1;
                    } else if grid[i][j] == Object::Square {
                        empty_row = i + 1;
                    }
                }
            }
        }
        Direction::Left => {
            for i in 0..grid.len() {
                let mut empty_col = 0;
                for j in 0..grid[i].len() {
                    if grid[i][j] == Object::Circle {
                        grid[i][j] = Object::Empty;
                        grid[i][empty_col] = Object::Circle;
                        empty_col += 1;
                    } else if grid[i][j] == Object::Square {
                        empty_col = j + 1;
                    }
                }
            }
        }
        Direction::Down => {
            for j in 0..grid[0].len() {
                let mut empty_row = grid.len() - 1;
                for i in (0..grid.len()).rev() {
                    if grid[i][j] == Object::Circle {
                        grid[i][j] = Object::Empty;
                        grid[empty_row][j] = Object::Circle;
                        if empty_row > 0 {
                            empty_row -= 1;
                        }
                    } else if grid[i][j] == Object::Square {
                        if i > 0 {
                            empty_row = i - 1;
                        }
                    }
                }
            }
        }
        Direction::Right => {
            for i in 0..grid.len() {
                let mut empty_col = grid[i].len() - 1;
                for j in (0..grid[i].len()).rev() {
                    if grid[i][j] == Object::Circle {
                        grid[i][j] = Object::Empty;
                        grid[i][empty_col] = Object::Circle;
                        if empty_col > 0 {
                            empty_col -= 1;
                        }
                    } else if grid[i][j] == Object::Square {
                        if j > 0 {
                            empty_col = j - 1;
                        }
                    }
                }
            }
        }
    }
}

fn calculate_load(grid: &Vec<Vec<Object>>) -> i64 {
    let mut load = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Object::Circle {
                load += (grid.len() as i64) - i as i64;
            }
        }
    }
    load
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let loop_number = 1000000000;

    // Set up the progress bar
    // let progress_bar = ProgressBar::new(loop_number as u64);
    // match ProgressStyle::default_bar().template(
    //     "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
    // ) {
    //     Ok(style) => progress_bar.set_style(style),
    //     Err(e) => {
    //         eprintln!("Error setting progress bar style: {}", e);
    //         return -1; // or handle the error in some other way
    //     }
    // }

    // Get grid
    let mut grid = parse_input(input);

    // Store the past grid states
    let mut cache: HashMap<Vec<Vec<Object>>, usize> = HashMap::new();

    // Loop through the grid until we find a cycle
    for i in 0..loop_number {
        if cache.contains_key(&grid) {
            // We found a cycle. Get the index of the previous encounter
            let previous_encounter_index = cache.get(&grid).unwrap();

            // Calculate the number of cycles between the last and current state
            let cycle_length = i - previous_encounter_index;

            // Get the number of left over loops after the last occurence if this state
            let remaining_iterations = (loop_number - i) % cycle_length;

            // Get the index of the state that will be reached after the remaining iterations
            let target_index = previous_encounter_index + remaining_iterations;

            // Get the state at previous_encounter_index + target_index
            let target_state = cache.iter().find(|(_, &v)| v == target_index).unwrap().0;
            grid = target_state.clone();
            break;
        }

        cache.insert(grid.clone(), i);

        move_circles(&mut grid, Direction::Up);
        move_circles(&mut grid, Direction::Left);
        move_circles(&mut grid, Direction::Down);
        move_circles(&mut grid, Direction::Right);

        // // Update the progress bar
        // if i % 100 == 0 {
        //     progress_bar.inc(100);
        // }
    }

    let load = calculate_load(&grid);

    // Finish the progress bar
    // progress_bar.finish_with_message("Completed");
    calculate_load(&grid)
}
