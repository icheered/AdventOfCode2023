#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: Vec<Node>,
}

fn parse_input(input: &str) -> Map {
    let mut instructions = Vec::new();
    let mut nodes = Vec::new();
    

    // First line contains a string of L and R (e.g. LLRRLRLRLLLRLR)
    // Each character represents a direction to go in the tree
    // Subsequent non-empty lines contain nodes: FJT = (XDJ, LQV)
    // label = (left, right)
    for (index, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        if index == 0 {
            instructions = line.chars().map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            }).collect();
        } else {
            let mut parts = line.split(" = ");
            let label = parts.next().unwrap();
            let children = parts.next().unwrap();
            // Remove start and end character
            let children = &children[1..children.len()-1];
            let mut children = children.split(", ");
            let left = children.next().unwrap();
            let right = children.next().unwrap();
            nodes.push(Node {
                label: label.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            });
        }
    }
    

    Map {
        nodes,
        instructions,
    }
}


#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let map = parse_input(input);
    println!("Map: {:?}", map);

    // Start at label AAA. Follow the instructions to the next label, until the label ZZZ is reached. Repeat instructions if necessary.
    // Count the number of times the instructions are repeated.
    let mut current_label = "AAA".to_string();
    let mut count = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        let node = map.nodes.iter().find(|n| n.label == current_label).unwrap();
        let next_label = match map.instructions[count] {
            Direction::Left => node.left.clone(),
            Direction::Right => node.right.clone(),
        };
        if next_label == "ZZZ" {
            break;
        }
        
        current_label = next_label;
        
        count += 1;
        if count == map.instructions.len() {
            count = 0;
        }
    }

    steps as i64
}
