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

fn get_empty_rows(grid: &Vec<Vec<char>>) -> Vec<usize> {
    grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()
}

fn get_empty_columns(grid: &Vec<Vec<char>>) -> Vec<usize> {
    (0..grid[0].len())
        .filter(|&column| {
            grid
                .iter()
                .all(|row| row[column] == '.')
        })
        .collect::<Vec<usize>>()
}

// Get all the coordinates of '#' characters on the grid, return them as coordinates
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
    let grid = parse_input(input);
    let mut coordinates = get_coordinates(&grid);
    let empty_rows = get_empty_rows(&grid);
    let empty_columns = get_empty_columns(&grid);

    let expand_empty_to = 1000000-1;
    
    // Change coordinates to account for the empty rows and columns
    coordinates.iter_mut().for_each(|coordinate| {
        coordinate[1] += empty_rows
            .iter()
            .filter(|&row| *row < coordinate[1])
            .count() * expand_empty_to;
        
        coordinate[0] += empty_columns
            .iter()
            .filter(|&column| *column < coordinate[0])
            .count() * expand_empty_to;
    });

    // For every coordinate, calculate the distance to every other coordinate (delta x + delta y) and store the result in a vector
    let sum_distances = coordinates.iter().enumerate().map(|(i, coord1)| {
        coordinates[i+1..].iter().map(|coord2| {
            (coord1[0] as i64 - coord2[0] as i64).abs() + (coord1[1] as i64 - coord2[1] as i64).abs()
        }).sum::<i64>()
    }).sum::<i64>();

    // Return sum of distances
    sum_distances
}
