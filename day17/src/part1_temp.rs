use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Eq, Hash)]
struct Block {
    x: u16,
    y: u16,
    cost: u8,
    total_cost: u16,
    direction: Direction,
    steps: u8,
    previous: Option<Box<Block>>, // REMOVE FOR SPEED (only for plotting path)
}

// Implementing PartialEq, Eq, Ord, and PartialOrd to use Block in a BinaryHeap
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost.eq(&other.total_cost)
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering here because BinaryHeap is a max heap
        other.total_cost.cmp(&self.total_cost)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    // Every character is a number representing the cost for that grid
    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    // input
    //     .lines()
    //     .map(|line| {
    //         line.chars()
    //             .map(|c| c.to_digit(10).unwrap() as u8)
    //             .collect()
    //     })
    //     .collect()

    // Set first and last block to 0
    grid[0][0] = 0;
    let width = grid[0].len();
    let height = grid.len();
    grid[height - 1][width - 1] = 0;
    grid.clone()
}

fn get_neighbouring_blocks(grid: &Vec<Vec<u8>>, block: &Block) -> Vec<Block> {
    let mut neighbours = Vec::new();

    // Convert block.x and block.y to usize for indexing
    let x = block.x as usize;
    let y = block.y as usize;

    // Check if there is a block above
    if y > 0 {
        neighbours.push(Block {
            x: block.x,
            y: block.y - 1,
            cost: grid[y - 1][x],
            total_cost: 0,
            direction: Direction::None,
            steps: 0,
            previous: None, // REMOVE FOR SPEED (only for plotting path)
        });
    }
    // Check if there is a block to the left
    if x > 0 {
        neighbours.push(Block {
            x: block.x - 1,
            y: block.y,
            cost: grid[y][x - 1],
            total_cost: 0,
            direction: Direction::None,
            steps: 0,
            previous: None, // REMOVE FOR SPEED (only for plotting path)
        });
    }
    // Check if there is a block to the right
    if x < grid[0].len() - 1 {
        neighbours.push(Block {
            x: block.x + 1,
            y: block.y,
            cost: grid[y][x + 1],
            total_cost: 0,
            direction: Direction::None,
            steps: 0,
            previous: None, // REMOVE FOR SPEED (only for plotting path)
        });
    }

    // Check if there is a block below
    if y < grid.len() - 1 {
        neighbours.push(Block {
            x: block.x,
            y: block.y + 1,
            cost: grid[y + 1][x],
            total_cost: 0,
            direction: Direction::None,
            steps: 0,
            previous: None, // REMOVE FOR SPEED (only for plotting path)
        });
    }

    neighbours
}

fn is_reverse_direction(dir1: Direction, dir2: Direction) -> bool {
    match (dir1, dir2) {
        (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => true,
        (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => true,
        _ => false,
    }
}

pub fn solve(input: &str) -> i64 {
    let grid = parse_input(input); // Assuming parse_input is already defined

    let mut queue = BinaryHeap::new();
    queue.push(Block {
        x: 0,
        y: 0,
        cost: grid[0][0],
        total_cost: grid[0][0] as u16,
        direction: Direction::None,
        steps: 0,
        previous: None,
    });

    let mut visited = HashSet::new();

    while let Some(current_block) = queue.pop() {
        if visited.contains(&(current_block.x, current_block.y, current_block.direction)) {
            continue;
        }
        visited.insert((current_block.x, current_block.y, current_block.direction));

        if usize::from(current_block.x) == grid[0].len() - 1
            && usize::from(current_block.y) == grid.len() - 1
        {
            let total_cost = current_block.total_cost;

            let mut path = Vec::new();
            let mut block = Some(current_block);
            while let Some(current) = block {
                path.push((current.x, current.y));
                block = current.previous.map(|b| *b);
            }

            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    if path.contains(&(x as u16, y as u16)) {
                        print!("\x1b[0;31m{}\x1b[0m ", grid[y][x]);
                    } else {
                        print!("{} ", grid[y][x]);
                    }
                }
                println!();
            }
            return total_cost as i64;
        }

        for neighbour in get_neighbouring_blocks(&grid, &current_block) {
            let mut direction = Direction::None;
            if neighbour.x > current_block.x {
                direction = Direction::Right;
            } else if neighbour.x < current_block.x {
                direction = Direction::Left;
            } else if neighbour.y > current_block.y {
                direction = Direction::Down;
            } else if neighbour.y < current_block.y {
                direction = Direction::Up;
            }

            let steps = if direction == current_block.direction {
                current_block.steps + 1
            } else {
                1
            };

            if !(direction == current_block.direction && steps > 3)
                && !is_reverse_direction(direction, current_block.direction)
            {
                if !visited.contains(&(neighbour.x, neighbour.y, direction)) {
                    queue.push(Block {
                        x: neighbour.x,
                        y: neighbour.y,
                        cost: neighbour.cost,
                        total_cost: current_block.total_cost + neighbour.cost as u16,
                        direction: direction,
                        steps: steps,
                        previous: Some(Box::new(current_block.clone())), // Consider removing for speed
                    });
                }
            }
        }
    }

    0
}
