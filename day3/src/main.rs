fn get_multidigit_number(grid: Vec<Vec<char>>, row: usize, col: usize)-> usize {
    // Look left and right until a non-number is found
    let mut left = col;
    let mut right = col;
    while left > 0 && grid[row][left-1].is_numeric() {
        left -= 1;
    }
    while right < grid[0].len() - 1 && grid[row][right +1].is_numeric()   {
        right += 1;
    }

    // Get the number from the grid
    let mut number = String::new();
    for i in left..=right {
        number.push(grid[row][i]);
    }

    // Get the number from the string
    let number = number.parse::<usize>().unwrap();
    return number;
}

fn get_gear_ratio(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    // Create mutable copy
    let mut clean = grid.clone();

    // If character above or below is number, replace corners on that side with dots
    for i in -1..=1 {
        let new_row = row as isize + i;

        // Check if out of bounds
        if new_row < 0 || new_row >= grid.len() as isize || i == 0 {
            continue;
        }

        let new_row = new_row as usize;

        if grid[new_row][col].is_numeric() {
            for j in -1..=1 {
                let new_col = col as isize + j;

                // Check if out of bounds
                if new_col < 0 || new_col >= grid[0].len() as isize || j == 0 {
                    continue;
                }

                let new_col = new_col as usize;

                clean[new_row][new_col] = '.';
            }
        }
    }
    
    // Count the number of numbers around this character
    let mut count = 0;
    let mut number_locations: Vec<(usize, usize)> = Vec::new();
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

            // Check if number
            if clean[new_row][new_col].is_numeric() {
                count += 1;
                // Store the location of the number so I don't have to find it again later
                number_locations.push((new_row, new_col));
            }
        }
    }

    if count == 2 {
        // Get the numbers
        let numbers = number_locations.iter().map(|(row, col)| get_multidigit_number(grid.clone(), *row, *col)).collect::<Vec<usize>>();

        // Multiply the numbers
        let product = numbers.iter().product::<usize>();
        return product;
    } else {
        return 0;
    }

    
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Read the input as a 2D array
    let lines = include_str!("input.txt").lines();
    let grid: Vec<Vec<char>> = lines
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;
    let cloned_grid_because_rust_wants_it = grid.clone();
    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            // Check if gar
            if *c == '*' {
                let gear_ratio = get_gear_ratio(&cloned_grid_because_rust_wants_it, row, col);
                sum += gear_ratio;
            }
        }
    }
    // display_grid(&mask);

    println!("Sum: {}", sum);



    Ok(())
}
