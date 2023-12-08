use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

// We are cool and use hashmaps instead of vectors
#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}


fn parse_input(input: &str) -> Map {
    let mut instructions = Vec::new();
    let mut nodes = HashMap::new(); 
    
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
            let label = parts.next().unwrap().to_string();
            let children = parts.next().unwrap();
            let children = &children[1..children.len()-1];
            let mut children = children.split(", ");
            let left = children.next().unwrap().to_string();
            let right = children.next().unwrap().to_string();
            nodes.insert(label.clone(), Node { left, right });
        }
    }
    

    Map { instructions, nodes }
}


// These are basic functions so I stole them from the internet
fn greatest_common_denominator(a: &BigInt, b: &BigInt) -> BigInt {
    if *b == Zero::zero() {
        a.clone()
    } else {
        greatest_common_denominator(b, &(a % b))
    }
}

fn smallest_common_multiple(numbers: &[BigInt]) -> BigInt {
    numbers.iter().cloned().fold(One::one(), |acc, n| {
        let gcd = greatest_common_denominator(&acc, &n);
        acc * (n / gcd)
    })
}


#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let map = parse_input(input);
    // Starting nodes are nodes ending with A
    let starting_nodes: Vec<String> = map.nodes.keys().filter(|k| k.ends_with("A")).map(|k| k.clone()).collect();

    // Array to hold the minimal steps for each node
    let mut minimal_steps = Vec::new();

    /*
        Instead of looping through all nodes simultaneously until all nodes end in Z, loop through all nodes individually and find the minimal steps to reach a node ending with Z. Then we find the smallest common multiple of all minimal steps, which is the total number of steps needed for all nodes to end in Z.
        
        This is much faster. 

        (We ignore edgecases like "But if it takes 10 steps to hit Z, it doesn't mean that 20 steps also hit Z" and "The paths can hit Z at multiple different steps" because again, it is faster.)
    */

    // For each node, find the minimal steps to reach a node ending with Z
    for node_label in starting_nodes.iter() {
        let mut current_label = node_label;
        let mut instruction_count = 0;
        let mut steps = 0;
        loop {
            steps += 1;

            // Find the next node
            let node = map.nodes.get(current_label).unwrap();
            let next_label = match map.instructions[instruction_count] {
                Direction::Left => &node.left,
                Direction::Right => &node.right,
            };

            if next_label.ends_with("Z") {
                break;
            }

            instruction_count += 1;
            if instruction_count == map.instructions.len() {
                instruction_count = 0;
            }

            current_label = next_label;
        }
        //println!("Node: {}, steps: {}", node, steps);
        minimal_steps.push(steps);
    }
    //println!("Minimal steps: {:?}", minimal_steps);
    let minimal_steps_bigint: Vec<BigInt> = minimal_steps.iter().map(|n| BigInt::from(*n)).collect();
    let sum = smallest_common_multiple(&minimal_steps_bigint).to_string().parse::<i64>().unwrap();
    return sum;
}

