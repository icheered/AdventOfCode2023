#[derive(PartialEq)]
enum Squaretype {
    Empty,
    Number,
    Special,
}


fn ripple_clean(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
    let squaretype: Squaretype = match grid[row][col] {
        '.' => Squaretype::Empty,
        '1'..='9' => Squaretype::Number,
        _ => Squaretype::Special,
    };

    if squaretype == Squaretype::Empty {
        return;
    }
    
    // Replace index with dot
    grid[row][col] = '.';

    
    if squaretype == Squaretype::Special {
        // Recurivevely call for all non-dot characters around this character
        for i in -1..=1 {
            for j in -1..=1 {
                let new_row = row as isize + i;
                let new_col = col as isize + j;

                // Check if out of bounds
                if new_row < 0 || new_col < 0 || new_row >= grid.len() as isize || new_col >= grid[0].len() as isize {
                    continue;
                }

                let new_row = new_row as usize;
                let new_col = new_col as usize;

                // Check if dot
                if grid[new_row][new_col] != '.' {
                    // Call this function with that row/col
                    ripple_clean(grid, new_row, new_col);
                }
            }
        }
    } else {
        // Do the same but only left/right for numbers
        for j in -1..=1 {
            let new_row = row as isize;
            let new_col = col as isize + j;

            // Check if out of bounds
            if new_row < 0 || new_col < 0 || new_row >= grid.len() as isize || new_col >= grid[0].len() as isize {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            // Check if dot
            if grid[new_row][new_col].is_digit(10) {
                // Call this function with that row/col
                ripple_clean(grid, new_row, new_col);
            }
        }
    }
}


fn display_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

pub fn solve(input: &str) -> i32 {
    let lines = input.lines();
    // Read the input as a 2D array
    let grid: Vec<Vec<char>> = lines
        .map(|line| line.chars().collect())
        .collect();

    // Copy the grid to a mutable variable to hold the mask
    let mut mask = grid.clone();
    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            // Check if special character (not number or dot)
            if !c.is_numeric() && *c != '.' {
                ripple_clean(&mut mask, row, col);
            }
        }
    }
    // display_grid(&mask);

    // Loop over grid to collect all the numbers that are not in the mask
    // Number can be multiple characters
    let mut sum = 0;
    for (row, line) in grid.iter().enumerate() {
        let mut col = 0;
        while col < line.len() { // Use while instead of for to be able to skip over numbers
            let c = line[col];
            if c.is_numeric() && mask[row][col] == '.' {
                let mut number = String::new();
                let mut i = col;
                while i < line.len() && line[i].is_numeric() {
                    number.push(line[i]);
                    i += 1;
                }
                sum += number.parse::<usize>().unwrap();

                // Update col to skip over the number we just processed
                col = i;
            } else {
                col += 1;  // Increment col normally if the character is not numeric
            }
        }
    }
    return sum as i32;
}


