fn get_next_number(numbers: &Vec<i64>) -> i64 {
    // Create an array of the differences between each number
    let mut differences = Vec::new();
    for i in 0..numbers.len() - 1 {
        differences.push(numbers[i + 1] - numbers[i]);
    }

    // If the array is all zeroes, return the last number
    if differences.iter().all(|&x| x == 0) {
        return numbers[numbers.len() - 1];
    }

    // Otherwise call the function again with the differences to get the next number
    let increment = get_next_number(&differences);
    numbers[numbers.len() - 1] + increment
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let numbers = parse_input(input);
    
    // Get extrapolated values and sum them
    numbers
        .iter()
        .map(|numbers| get_next_number(numbers))
        .sum()

    
}
