fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn solve(input: &str) -> i32 {
    let mut total: u32 = 0;
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
    for line in input.lines() {
        let mut first_number_str: char = ' ';
        let mut last_number_str: char = ' ';

        // Loop through the line to find the first number
        let mut left_string: String = String::new();
        let mut right_string: String = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                first_number_str = c;
                break;
            }

            // Append characters to the left string
            left_string.push(c);
            
            // Check if the left_string contains a word in the numbers array
            for (index, number) in numbers.iter().enumerate() {
                if left_string.contains(number) {
                    first_number_str = (index + 1).to_string().chars().next().unwrap();
                    break;
                }
            }
            if first_number_str != ' ' {
                break;
            }
            
        }
        // Loop through the line in reverse to find the last number
        for c in line.chars().rev() { 
            if c.is_numeric() {
                last_number_str = c;
                break;
            }

            // Append characters to the left string
            right_string.push(c);
            
            // Check if the right_string reversed contains a word in the numbers array
            for (index, number) in numbers.iter().enumerate() {
                if reverse(&right_string).contains(number) {
                    last_number_str = (index + 1).to_string().chars().next().unwrap();
                    break;
                }
            }
            if last_number_str != ' ' {
                break;
            }
        }

        let number_str: String = first_number_str.to_string() + &last_number_str.to_string();
        
        // Check if the number is valid
        if number_str.parse::<u32>().is_err() {
            continue;
        }
        let number: u32 = number_str.parse().unwrap();
        //println!("{} ", number);
        total += number;
    }
    return total as i32;
}