fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut total: u32 = 0;
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
    for line in include_str!("input.txt").lines() {
        let mut first_number_str: char = ' ';
        let mut last_number_str: char = ' ';

        // Loop through the line to find the first number
        let mut leftString: String = String::new();
        let mut rightString: String = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                first_number_str = c;
                break;
            }

            // Append characters to the left string
            leftString.push(c);
            
            // Check if the leftString contains a word in the numbers array
            for (index, number) in numbers.iter().enumerate() {
                if leftString.contains(number) {
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
            rightString.push(c);
            
            // Check if the rightString reversed contains a word in the numbers array
            for (index, number) in numbers.iter().enumerate() {
                if reverse(&rightString).contains(number) {
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
    println!("Total: {}", total);
    Ok(())
}