#[derive(Debug)]
struct Card {
    card_number: u32,
    own_numbers: Vec<u32>,
    winning_numbers: Vec<u32>
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

fn get_card_score(card: &Card) -> u32 {
    let mut score: u32 = 0;
    for own_number in &card.own_numbers {
        for winning_number in &card.winning_numbers {
            if own_number == winning_number {
                score += 1;
            }
        }
    }
    score
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i32 {
    let mut cards = parse_input(input);
    // Create a new vector with 1s with length of cards
    let mut counts: Vec<u32> = vec![1; cards.len()];
    
    // Enumerate over cards so I have access to index
    cards.iter_mut().enumerate().for_each(|(i, card)| {
        // Get score for card
        let score = get_card_score(card);

        // Increment the 'count' of the next <score> cards
        if score > 0 {
            for j in (i+1)..(i+1+score as usize) {
                counts[j] += counts[i];
            }
        }
        
    });

    // Sum the counts vector
    counts.iter().sum::<u32>() as i32
}
