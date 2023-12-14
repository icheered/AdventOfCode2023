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
    // Move circle in direction until it hits another circle or square
    match direction {
        Direction::Up => {
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid[i][j] == Object::Circle {
                        if i == 0 {
                            // Circle is at the top of the grid, so it can't move up
                            continue;
                        }

                        // Move circle in up direction until it hits another circle or square
                        let mut k = i;
                        while k > 0 && grid[k - 1][j] == Object::Empty {
                            grid[k][j] = Object::Empty;
                            grid[k - 1][j] = Object::Circle;
                            k -= 1;
                        }
                    }
                }
            }
        }
        Direction::Left => {
            // Loop through the grid from left to right, top to bottom
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid[i][j] == Object::Circle {
                        if j == 0 {
                            // Circle is at the left of the grid, so it can't move left
                            continue;
                        }

                        // Move circle in left direction until it hits another circle or square
                        let mut k = j;
                        while k > 0 && grid[i][k - 1] == Object::Empty {
                            grid[i][k] = Object::Empty;
                            grid[i][k - 1] = Object::Circle;
                            k -= 1;
                        }
                    }
                }
            }
        }
        Direction::Down => {
            // Loop through the grid from bottom to top, left to right
            for i in (0..grid.len()).rev() {
                for j in (0..grid[i].len()).rev() {
                    if grid[i][j] == Object::Circle {
                        if i == grid.len() - 1 {
                            // Circle is at the bottom of the grid, so it can't move down
                            continue;
                        }

                        // Move circle in down direction until it hits another circle or square
                        let mut k = i;
                        while k < grid.len() - 1 && grid[k + 1][j] == Object::Empty {
                            grid[k][j] = Object::Empty;
                            grid[k + 1][j] = Object::Circle;
                            k += 1;
                        }
                    }
                }
            }
        }
        Direction::Right => {
            // Loop through the grid from right to left, bottom to top
            for i in (0..grid.len()).rev() {
                for j in (0..grid[i].len()).rev() {
                    if grid[i][j] == Object::Circle {
                        if j == grid[i].len() - 1 {
                            // Circle is at the right of the grid, so it can't move right
                            continue;
                        }

                        // Move circle in right direction until it hits another circle or square
                        let mut k = j;
                        while k < grid[i].len() - 1 && grid[i][k + 1] == Object::Empty {
                            grid[i][k] = Object::Empty;
                            grid[i][k + 1] = Object::Circle;
                            k += 1;
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
