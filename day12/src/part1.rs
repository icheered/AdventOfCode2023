#[derive(Debug, Eq, Hash, Copy, PartialEq, Clone)]
enum Spring {
    Ok,
    Broken,
    Unknown
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Row {
    groups: Vec<i32>,
    springs: Vec<Spring>,
    permutations: u64
}

fn parse_input(input: &str) -> Vec<Row> {
    input.lines().map(|line| {
        let mut parts = line.split(' ');
        let springs = parts.next().unwrap()
            .chars()
            .map(|c| match c {
                '.' => Spring::Ok,
                '?' => Spring::Unknown,
                '#' => Spring::Broken,
                _ => panic!("Unknown spring type"),
            })
            .collect::<Vec<_>>();

        let groups = parts.next().unwrap()
            .split(',')
            .map(|s| s.parse().expect("Invalid number"))
            .collect::<Vec<_>>();

        Row { 
            springs,
            groups,
            permutations: 1
        }
    }).collect()
}


fn is_valid_sequence(springs: &[Spring], groups: &[i32]) -> bool {
    fn helper(springs: &[Spring], groups: &[i32], group_index: usize, count: usize) -> bool {
        
        if group_index >= groups.len() {
            return springs.iter().all(|&s| s != Spring::Broken);
        }
        // Print strings
        println!("\nsprings: {:?}", springs);
        // Print groups from group_index
        println!("groups: {:?}", &groups[group_index..]);

        let mut broken_count = 0;
        let mut i = 0;
        while i < springs.len() {
            match springs[i] {
                Spring::Broken => broken_count += 1,
                Spring::Ok => {
                    if broken_count > 0 {
                        break;
                    }
                }
                Spring::Unknown => {
                    // Check if the unknown spring can be part of the current group
                    println!("Checking if unknown spring can be part of the current group");
                    if helper(&springs[i + 1..], groups, group_index, count + broken_count + 1) {
                        return true;
                    }
                    // Otherwise, treat it as an Ok and break
                    break;
                }
            }
            i += 1;
        }

        let group_size = groups[group_index] as usize;
        let valid = broken_count == count + group_size && helper(&springs[i..], groups, group_index + 1, 0);
        println!("Valid: {:?}", valid);
        valid
    }

    helper(springs, groups, 0, 0)
}


fn get_all_permutations(springs: &[Spring], groups: &[i32]) -> Vec<Vec<Spring>> {
    if let Some(i) = springs.iter().position(|&s| s == Spring::Unknown) {
        let mut permutations = Vec::new();

        let mut springs_ok = springs.to_vec();
        springs_ok[i] = Spring::Ok;
        if is_valid_sequence(&springs_ok, groups) {
            permutations.extend(get_all_permutations(&springs_ok, groups));
        }

        let mut springs_broken = springs.to_vec();
        springs_broken[i] = Spring::Broken;

        if is_valid_sequence(&springs_broken, groups) {
            permutations.extend(get_all_permutations(&springs_broken, groups));
        }

        permutations
    } else {
        if is_valid_sequence(springs, groups) {
            vec![springs.to_vec()]
        } else {
            Vec::new()
        }
    }
}



#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let mut rows = parse_input(input);
    for row in &mut rows {
        if !row.springs.is_empty() {
            // Start with group_index = 0 and broken_count = 0
            let permutations = get_all_permutations(&row.springs, &row.groups);
            row.permutations *= permutations.len() as u64;
        }
    }

    for row in &rows {
        println!("{:?}", row.permutations);
    }

    rows.iter().map(|row| row.permutations).sum::<u64>() as i64
}
