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
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            // Extract the hexadecimal code from the line
            let hex_code = line.split_whitespace().last().unwrap();
            let hex_code = &hex_code[2..8]; // Skip the '#' character

            // Parse the distance (first five digits)
            let distance_str = &hex_code[..5];
            let distance = i64::from_str_radix(distance_str, 16).expect("Invalid hex for distance");

            // Parse the direction (last digit)
            let direction_digit = hex_code.chars().last().unwrap();
            let direction = match direction_digit {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("Invalid direction code"),
            };

            Instruction {
                direction,
                distance,
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
