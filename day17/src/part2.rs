#![allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Block {
    cost: u8,
    total_cost: u16,
    direction: Direction,
    steps: u8,
}

fn parse_input(input: &str) -> Vec<Vec<Block>> {
    // Every character is a number representing the cost for that grid
    // Initialize direction as None and setps as 0
    // x and y are the coordinates of the block

    // Get width and height based on number of lines
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    // Initialize grid with empty blocks
    let mut grid = vec![
        vec![
            Block {
                cost: 0,
                total_cost: 0,
                direction: Direction::None,
                steps: 0,
            };
            width
        ];
        height
    ];

    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid[y][x].cost = c.to_digit(10).unwrap() as u8;
        });
    });
    grid
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let grid = parse_input(input);

    for row in &grid {
        for block in row {
            print!("{:?} ", block.cost);
        }
        println!();
    }

    0
}
