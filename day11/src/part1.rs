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

fn expand_space(grid: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Every empty column should become 2 columns
    // Every empty row should become 2 rows
    
    // Get rows that are empty
    let empty_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    
    let empty_columns = (0..grid[0].len())
        .filter(|&column| {
            grid
                .iter()
                .all(|row| row[column] == '.')
        })
        .collect::<Vec<usize>>();

    
    println!("Empty rows: {:?}", empty_rows);
    println!("Empty columns: {:?}", empty_columns);
    
    // Loop backwards through the rows and columns, inserting empty columns and rows
    for row in empty_rows.iter().rev() {
        grid.insert(*row, vec!['.'; grid[0].len()]);
    }

    for column in empty_columns.iter().rev() {
        for row in &mut *grid {
            row.insert(*column, '.');
        }
    }

    // Return the grid
    grid.to_vec()
}

// Get all the coordinates of '#' characters on the grid, return them as a vector of size 2 vectors
fn get_coordinates(grid: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut coordinates = Vec::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '#' {
                coordinates.push(vec![x, y]);
            }
        }
    }

    coordinates
}


#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let mut grid = parse_input(input);

    // Expand the grid
    grid = expand_space(&mut grid);

    // Get all the coordinates of '#' characters on the grid
    let coordinates = get_coordinates(&grid);
    println!("Coordinates: {:?}", coordinates);

    // For every coordinate, calculate the distance to every other coordinate (delta x + delta y) and store the result in a vector
    let mut distances = Vec::new();
    for (i, coordinate) in coordinates.iter().enumerate() {
        for j in (i+1)..coordinates.len() {
            let distance = (coordinate[0] as i64 - coordinates[j][0] as i64).abs() + (coordinate[1] as i64 - coordinates[j][1] as i64).abs();
            distances.push(distance);
        }
    }

    // Calculate sum of distances
    let sum = distances.iter().sum::<i64>();
    
    sum
}
