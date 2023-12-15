#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    // Split the input on commas
    // For every item: For every character in the input, add it to the sum, multiply by 17, and get remainder after dividing by 255
    // Then sum all of the items
    input
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|item| item.chars().fold(0, |acc, c| ((acc + c as i64) * 17) % 256))
        .sum::<i64>() as i64
}
