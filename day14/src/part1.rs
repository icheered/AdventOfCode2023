#[derive(PartialEq)]
enum Object {
    Circle,
    Square,
    Empty
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
                _ => panic!("Invalid input")
            }
        }
        grid.push(row);
    }
    grid
}

fn move_circles_up(grid: &mut Vec<Vec<Object>>) {
    // Go through the grid and move all circles up until there is another circle or square above it
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Object::Circle {
                if i == 0 {
                    // Circle is at the top of the grid, so it can't move up
                    continue;
                }
                
                // Move circle in direction until it hits another circle or square
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
    let mut grid = parse_input(input);
    
     
    move_circles_up(&mut grid);
    for line in &grid {
        for c in line {
            match c {
                Object::Circle => print!("O"),
                Object::Square => print!("#"),
                Object::Empty => print!(".")
            }
        }
        println!();
    }

    let load = calculate_load(&grid);
    println!("Load: {}", load);
    0
}
