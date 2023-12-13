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

        let mut repeated_springs = Vec::new();
        for _ in 0..5 {
            repeated_springs.extend_from_slice(&springs);
            repeated_springs.push(Spring::Unknown); // Add Unknown spring in between
        }
        repeated_springs.pop(); // Remove the last Unknown spring

        // Add an OK spring at the start and end of the sequence
        let mut final_springs = vec![Spring::Ok];
        final_springs.append(&mut repeated_springs);
        final_springs.push(Spring::Ok);

        Row { 
            springs: final_springs,
            groups: groups.repeat(5),
            permutations: 1
        }
    }).collect()
}


fn is_valid_sequence(springs: &[Spring], groups: &[i32], group_index: usize, broken_count: usize) -> bool {
    if group_index >= groups.len() {
        return broken_count == 0;
    }

    if broken_count > groups[group_index] as usize {
        return false;
    }

    true
}

fn get_all_permutations(springs: &[Spring], groups: &[i32], group_index: usize, broken_count: usize) -> Vec<Vec<Spring>> {
    if let Some(i) = springs.iter().position(|&s| s == Spring::Unknown) {
        let mut permutations = Vec::new();

        let mut springs_ok = springs.to_vec();
        springs_ok[i] = Spring::Ok;
        if is_valid_sequence(&springs_ok, groups, group_index, broken_count) {
            permutations.extend(get_all_permutations(&springs_ok, groups, group_index, broken_count));
        }

        let mut springs_broken = springs.to_vec();
        springs_broken[i] = Spring::Broken;
        let new_broken_count = broken_count + 1;
        let new_group_index = if new_broken_count == groups[group_index] as usize {
            group_index + 1
        } else {
            group_index
        };

        if is_valid_sequence(&springs_broken, groups, new_group_index, new_broken_count % (groups[group_index] as usize)) {
            permutations.extend(get_all_permutations(&springs_broken, groups, new_group_index, new_broken_count % (groups[group_index] as usize)));
        }

        permutations
    } else {
        if is_valid_sequence(springs, groups, group_index, broken_count) {
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
            let permutations = get_all_permutations(&row.springs, &row.groups, 0, 0);
            row.permutations *= permutations.len() as u64;
        }
    }

    rows.iter().map(|row| row.permutations).sum::<u64>() as i64
}
