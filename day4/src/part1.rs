#[derive(Debug)]
struct Card {
    card_number: u32,
    own_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

fn parse_input(input: &str) -> Vec<Card> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split(": ").collect();
        let card_number: u32 = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();

        let numbers: Vec<&str> = parts[1].split(" | ").collect();
        let winning_numbers: Vec<u32> = numbers[0].split_whitespace().map(|n| n.parse().unwrap()).collect();
        let own_numbers: Vec<u32> = numbers[1].split_whitespace().map(|n| n.parse().unwrap()).collect();

        Card {
            card_number,
            own_numbers,
            winning_numbers,
        }
    }).collect()
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i32 {
    let cards = parse_input(input);
    let total = cards.iter().map(|card| {
        let mut score: i32 = 0;
        for own_number in &card.own_numbers {
            for winning_number in &card.winning_numbers {
                if own_number == winning_number {
                    score += 1;
                    println!("Card {} matches own {} to {}", card.card_number, own_number, winning_number);
                }
            }
        }
        if score > 0 {
            let points = i32::pow(2, (score-1).try_into().unwrap());
            println!("Card {} has score {} - points {}", card.card_number, score, points);
            return points;
        } else {
            return 0 as i32;
        }
        
        
    }).sum::<i32>();
    total as i32
}
