
fn parse_input(input: &str) -> Vec<Vec<Vec<Object>>> {
    let mut patterns = Vec::new();
    let mut current_pattern = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(current_pattern);
            current_pattern = Vec::new();
        } else {
            let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(Object::Ash),
                '#' => row.push(Object::Rock),
                _ => panic!("Unknown character: {}", c),
            }
        }
        
            current_pattern.push(row);
        }
    }
    if !current_pattern.is_empty() {
        patterns.push(current_pattern);
    }
    patterns
}


#[derive(Debug, Eq, Hash, Copy, PartialEq, Clone)]
enum Object {
    Ash,
    Rock,
}

fn get_number_of_differences(row1: &Vec<Object>, row2: &Vec<Object>) -> i32 {
    let mut differences = 0;
    for i in 0..row1.len() {
        if row1[i] != row2[i] {
            differences += 1;
        }
    }
    differences
}


fn get_mirror_index_horizontal(pattern: &Vec<Vec<Object>>) -> i32 {
    // For each row, check if the next row is the same
    // If it is, look up and down to see if these are the same
    // Continue until you reach the edge of the pattern
    for i in 0..pattern.len() {
        let row = &pattern[i];
        if i + 1 < pattern.len() {
            let next_row = &pattern[i + 1];
            // Get number of differneces between row and next_row
            let mut differences = get_number_of_differences(row, next_row);
            if differences <= 1 {
                let mut found_mirror = true;
                let mut mirror_index = 1;
                while found_mirror {
                    if i >= mirror_index && i + 1 + mirror_index < pattern.len() {
                        let row_above = &pattern[i - mirror_index];
                        let row_below = &pattern[i + 1 + mirror_index];
                        differences += get_number_of_differences(row_above, row_below);
                        if differences > 1 {
                            found_mirror = false;
                        }
                    } else {
                        if differences == 1 {
                            return (i as i32) + 1;
                        } else {
                            found_mirror = false;
                        }
                    }
                    if found_mirror {
                        mirror_index += 1;
                    }
                }
            }
        }
    }
    0
}

fn transpose(matrix: Vec<Vec<Object>>) -> Vec<Vec<Object>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let row_length = matrix.len();
    let col_length = matrix[0].len();

    let mut transposed = vec![vec![Object::Ash; row_length]; col_length]; // Adjust the default value if needed

    for (i, row) in matrix.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            transposed[j][i] = item;
        }
    }

    transposed
}





#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let patterns = parse_input(input);
    // for pattern in &patterns {
    //     for row in pattern {
    //         for object in row {
    //             match object {
    //                 Object::Ash => print!("."),
    //                 Object::Rock => print!("#"),
    //             }
    //         }
    //         println!()
    //     }
    //     println!();
    //     println!("Transposed:");
    //     for row in transpose(pattern.to_vec()) {
    //         for object in row {
    //             match object {
    //                 Object::Ash => print!("."),
    //                 Object::Rock => print!("#"),
    //             }
    //         }
    //         println!()
    //     }
    // }

    // let mirror_indices_horizontal = patterns.iter().map(|pattern| get_mirror_index_horizontal(pattern)).collect::<Vec<i32>>();
    // let horizontal_sum = mirror_indices_horizontal.iter().sum::<i32>();
    // let mirror_indices_vertical = patterns.iter().map(|pattern| get_mirror_index_horizontal(&transpose(pattern.to_vec()))).collect::<Vec<i32>>();
    // let vertical_sum = mirror_indices_vertical.iter().sum::<i32>();


    // println!("Horizontal: {:?}", mirror_indices_horizontal);
    // println!("Vertical:   {:?}", mirror_indices_vertical);
    // println!("Horizontal sum: {}", horizontal_sum);
    // println!("Vertical sum: {}", vertical_sum);






    let sum = patterns.iter().map(|pattern| {
        // Return get_mirror_index_horizontal(pattern) if it is non-zero, otherwise return get_mirror_index_horizontal(transpose(pattern.to_vec()))
        let horizontal = get_mirror_index_horizontal(pattern);
        if horizontal != 0 {
            horizontal * 100
        } else {
            get_mirror_index_horizontal(&transpose(pattern.to_vec()))
        }
    }).sum::<i32>();

    return sum as i64;
}
