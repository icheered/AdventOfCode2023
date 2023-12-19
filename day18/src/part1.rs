#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Color(u8, u8, u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    distance: i64,
    color: Color,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction = match parts.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let distance = parts.next().unwrap().parse().unwrap();
            let color = parts.next().unwrap();

            // Parse color, it has the following format: (#7a21e3)
            let color = Color(
                u8::from_str_radix(&color[2..4], 16).unwrap(),
                u8::from_str_radix(&color[4..6], 16).unwrap(),
                u8::from_str_radix(&color[6..8], 16).unwrap(),
            );

            Instruction {
                direction,
                distance,
                color,
            }
        })
        .collect()
}
fn count_border_points(vertices: &Vec<(i64, i64)>) -> i64 {
    let mut b = 0;
    for i in 0..vertices.len() - 1 {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[i + 1];

        // Count the points on the line segment
        if x1 == x2 {
            // Vertical line
            b += (y1 - y2).abs();
        } else if y1 == y2 {
            // Horizontal line
            b += (x1 - x2).abs();
        }
    }
    b
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let instructions = parse_input(input);
    //println!("{:?}", instructions);

    // Start at 0,0
    // Loop over the instructions and store the new coordinates

    let mut vertices: Vec<(i64, i64)> = Vec::new();
    vertices.push((0, 0));

    for instruction in instructions {
        let (x, y) = *vertices.last().unwrap();
        //println!("x: {}, y: {}", x, y);
        let (x, y) = match instruction.direction {
            Direction::Up => (x, y - instruction.distance),
            Direction::Down => (x, y + instruction.distance),
            Direction::Left => (x - instruction.distance, y),
            Direction::Right => (x + instruction.distance, y),
        };
        vertices.push((x, y));
    }
    vertices.push(vertices[0]);

    // Reverse vertices
    vertices.reverse();

    // Use the shoelace algorithm to calculate the area
    let mut area = 0;
    for i in 0..vertices.len() - 1 {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[i + 1];
        area += x1 * y2 - x2 * y1;
    }

    let absolute_area = (area / 2).abs();

    // Calculate border points
    let border_points = count_border_points(&vertices);
    absolute_area + border_points / 2 + 1
}
