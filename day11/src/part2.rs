#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Galaxy,
}

fn parse_input(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars().map(|c| {
                match c {
                    '.' => Space::Empty,
                    '#' => Space::Galaxy,
                    _ => panic!("Invalid character in input: {}", c),
                }
            }).collect::<Vec<Space>>()
        }).collect::<Vec<Vec<Space>>>()
}
fn get_empty_rows(grid: &Vec<Vec<Space>>) -> Vec<usize> {
    grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == Space::Empty))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()
}

fn get_empty_columns(grid: &Vec<Vec<Space>>) -> Vec<usize> {
    (0..grid[0].len())
        .filter(|&column| {
            grid
                .iter()
                .all(|row| row[column] == Space::Empty)
        })
        .collect::<Vec<usize>>()
}
// Get all the coordinates of '#' characters on the grid, return them as coordinates
fn get_coordinates(grid: &Vec<Vec<Space>>) -> Vec<(i64,i64)> {
    let mut coordinates = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if *space == Space::Galaxy {
                coordinates.push((x as i64, y as i64));
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
        coordinate.1 += empty_rows
            .iter()
            .filter(|&row| *row < coordinate.1 as usize)
            .count() as i64 * expand_empty_to;
        
        coordinate.0 += empty_columns
            .iter()
            .filter(|&column| *column < coordinate.0 as usize)
            .count() as i64 * expand_empty_to;
    });

    // For every coordinate, calculate the distance to every other coordinate (delta x + delta y) and store the result in a vector
    coordinates.iter().enumerate().map(|(i, coord1)| {
        coordinates[i + 1..].iter().map(|coord2| {
            (coord1.0 - coord2.0).abs() + (coord1.1 - coord2.1).abs()
        }).sum::<i64>()
    }).sum()

}
