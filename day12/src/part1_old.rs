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
        let mut springs = parts.next().unwrap().chars().map(|c| {
            match c {
                '.' => Spring::Ok,
                '?' => Spring::Unknown,
                '#' => Spring::Broken,
                _ => panic!("Unknown spring type"),
            }
        }).collect::<Vec<_>>();

        // Add an OK to the start and end of the springs
        springs.insert(0, Spring::Ok);
        springs.push(Spring::Ok);

        let groups_part = parts.next().unwrap();

        let groups = groups_part
            .split(',')
            .map(|s| s.parse::<i32>().expect("Invalid number"))
            .collect::<Vec<_>>();

        Row { groups, springs, permutations: 1 }
    }).collect()
}



// fn remove_logical_groups(rows: &mut Vec<Row>) {
//     for row in rows.iter_mut() {
//         // Compare length of last group to last number in groups
//         // If they are the same, remove the last group
//         while let (Some(&last_group_len), Some(last_spring_group)) = (row.groups.last(), row.springs.last()) {
//             if last_group_len == last_spring_group.len() as i32 {
//                 row.groups.pop();
//                 row.springs.pop();
//             } else if last_group_len + 1 == last_spring_group.len() as i32 {
//                 if last_spring_group.first() == Some(&Spring::Unknown) &&
//                     last_spring_group.last() == Some(&Spring::Unknown) {
//                     row.permutations *= 2;
//                 }
//                 row.groups.pop();
//                 row.springs.pop();
//             } else {
//                 break; 
//             }
//         }

//         // Process the first group
//         while let (Some(&first_group_len), Some(first_spring_group)) = (row.groups.first(), row.springs.first()) {
//             if first_group_len == first_spring_group.len() as i32 {
//                 row.groups.remove(0);
//                 row.springs.remove(0);
//             } else if first_group_len + 1 == first_spring_group.len() as i32 {
//                 if first_spring_group.first() == Some(&Spring::Unknown) &&
//                     first_spring_group.last() == Some(&Spring::Unknown) {
//                     row.permutations *= 2;
//                 }
//                 row.groups.remove(0);
//                 row.springs.remove(0);
//             } else {
//                 break; 
//             }
//         }
//     }
// }

fn is_valid_sequence(springs: &Vec<Spring>, groups: &Vec<i32>) -> bool {
    // Check if the row is valid by getting the length of the first sequence of Broken's
    let mut broken_count = 0;
    let mut group_index = 0;
    //println!("Checking if valid sequence: {:?} {:?}", springs, groups);
    for spring in springs.iter() {
        if spring == &Spring::Unknown {
            //println!("Unknown spring, valid");
            return true;
        } else if spring == &Spring::Broken {
            broken_count += 1;
        } else if spring == &Spring::Ok {
            if broken_count == 0 {
                continue;
            }
            if group_index >= groups.len() {
                //println!("Group index out of bounds, invalid");
                return false;
            }
            // Compare broken_count to first group
            if broken_count != groups[group_index] {
                //println!("Broken count wrong, invalid");
                return false;
            }
            broken_count = 0;
            group_index += 1;
        }
    }
    if group_index < groups.len() {
        //println!("Group index too low, invalid");
        return false;
    }
    //println!("Valid");
    true
}

fn get_all_permutations(springs: Vec<Spring>, groups: Vec<i32>) -> Vec<Vec<Spring>>{
    let mut permutations = Vec::new();

    // Check if there are any unknown springs
    // if let Some(i) = springs.iter().position(|&s| s == Spring::Unknown) {
    //     // Try setting the unknown spring to Ok
    //     let mut springs_ok = springs.clone();
    //     springs_ok[i] = Spring::Ok;
    //     if is_valid_sequence(&springs_ok, &groups) {
    //         permutations.extend(get_all_permutations(springs_ok, groups.clone()));
    //     }

    //     // Try setting the unknown spring to Broken
    //     let mut springs_broken = springs.clone();
    //     springs_broken[i] = Spring::Broken;
    //     if is_valid_sequence(&springs_broken, &groups) {
    //         permutations.extend(get_all_permutations(springs_broken, groups.clone()));
    //         //println!("Permutations: {:?}", permutations);
    //     }
    // } else {
    //     // No unknown springs, check if the sequence is valid
    //     if is_valid_sequence(&springs, &groups) {
    //         permutations.extend(springs);
    //     }
    // }
    // permutations

    if let Some(i) = springs.iter().position(|&s| s == Spring::Unknown) {
        // Try setting the unknown spring to Ok
        let mut springs_ok = springs.clone();
        springs_ok[i] = Spring::Ok;
        if is_valid_sequence(&springs_ok, &groups) {
            permutations.extend(get_all_permutations(springs_ok, groups.clone()));
        }

        // Try setting the unknown spring to Broken
        let mut springs_broken = springs.clone();
        springs_broken[i] = Spring::Broken;
        if is_valid_sequence(&springs_broken, &groups) {
            permutations.extend(get_all_permutations(springs_broken, groups.clone()));
            //println!("Permutations: {:?}", permutations);
        }
    } else {
        // No unknown springs, check if the sequence is valid
        if is_valid_sequence(&springs, &groups) {
            permutations.push(springs);
        }
    }
    permutations
}




#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let mut rows = parse_input(input);
    // Count sum of length of springs
    // let mut sum_before = 0;
    // for row in rows.iter() {
    //     sum_before += row.springs.len();
    // }
    


    // Remove logical groups
    // remove_logical_groups(&mut rows);
    // let mut sum_after = 0;
    // for row in rows.iter() {
    //     sum_after += row.springs.len();
    // }
    ////println!("Removed {}/{} Groups", sum_before - sum_after, sum_before);


    // Concatenate each spring group with a '.' in between
    // for row in rows.iter_mut() {
    //     for spring_group in row.springs.iter_mut() {
    //         spring_group.push(Spring::Ok);
    //         spring_group.insert(0, Spring::Ok);
    //     }
    // }

    // For each row, get the permutations and multiply it by the group.permutations
    for row in rows.iter_mut() {
        if row.springs.len() > 0 {
            let permutations = get_all_permutations(row.springs.clone(), row.groups.clone());
            //println!("ALL PERMUTATIONS: {:?}", permutations);
            row.permutations *= permutations.len() as u64;
        }
    }

    for row in rows.iter() {
        //println!("{:?}", row.permutations);
    }


    // Sum all permutations
    let sum = rows.iter().fold(0, |acc, row| acc + row.permutations);
    ////println!("Sum: {}", sum);
    return sum as i64;
}
