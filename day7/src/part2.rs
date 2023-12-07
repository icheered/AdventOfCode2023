use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<i32>,
    bid: i32,
    hand_type: Type,
}

fn get_type(cards: &Vec<char>) -> Type {
    let mut card_count: HashMap<char, i32> = HashMap::new();
    for card in cards.iter() {
        let count = card_count.entry(*card).or_insert(0);
        *count += 1;
    }

    let mut card_count_vec: Vec<i32> = Vec::new();
    for count in card_count.values() {
        card_count_vec.push(*count);
    }
    card_count_vec.sort();

    let joker_count = card_count.get(&'J').unwrap_or(&0);

    let best_hand_without_jokers = match card_count_vec.as_slice() {
        [1, 1, 1, 1, 1] => Type::HighCard,
        [1, 1, 1, 2] => Type::OnePair,
        [1, 2, 2] => Type::TwoPair,
        [1, 1, 3] => Type::ThreeOfAKind,
        [2, 3] => Type::FullHouse,
        [1, 4] => Type::FourOfAKind,
        [5] => Type::FiveOfAKind,
        _ => panic!("Invalid hand")
    };

    match (&best_hand_without_jokers, joker_count) {
        (Type::HighCard, 1) => Type::OnePair,
        (Type::OnePair, 1) => Type::ThreeOfAKind,
        (Type::OnePair, 2) => Type::ThreeOfAKind,
        (Type::TwoPair, 1) => Type::FullHouse,
        (Type::TwoPair, 2) => Type::FourOfAKind,
        (Type::ThreeOfAKind, 1) => Type::FourOfAKind,
        (Type::ThreeOfAKind, 2) => Type::FiveOfAKind,
        (Type::ThreeOfAKind, 3) => Type::FourOfAKind,
        (Type::FullHouse, 2) => Type::FiveOfAKind,
        (Type::FullHouse, 3) => Type::FiveOfAKind,
        (Type::FourOfAKind, 1) => Type::FiveOfAKind,
        (Type::FourOfAKind, 4) => Type::FiveOfAKind,
        _ => best_hand_without_jokers,
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    let lines: Vec<&str> = input.lines().collect();
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines.iter() {
        let mut split_line = line.split_whitespace();

        let cards: Vec<char> = split_line.next().unwrap().chars().collect();
        
        // Turn cards into numbers based on their value
        let mut cards_by_number: Vec<i32> = Vec::new();
        for card in cards.iter() {
            let card_number = match card {
                'J' => 1,
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'T' => 10,
                _ => card.to_digit(10).unwrap() as i32,
            };
            cards_by_number.push(card_number);
        }

        let bid: i32 = split_line.next().unwrap().parse().unwrap();
        let hand_type = get_type(&cards);

        hands.push(Hand { cards: cards_by_number, bid, hand_type });
    }
    
    hands
}


#[allow(unused_variables)]
pub fn solve(input: &str) -> i32 {
    let mut hands = parse_input(input);
    
    // Sort hands by hand_type, then by cards
    hands.sort_by(|a, b| {
        match a.hand_type.cmp(&b.hand_type) {
            std::cmp::Ordering::Equal => a.cards.cmp(&b.cards),
            other => other,
        }
    });

    for hand in hands.iter() {
        println!("{:?}", hand);
    }

    // Calculate total score by multiplying index+1 by bid
    let total_score = hands.iter().enumerate().fold(0, |acc, (index, hand)| acc + (index as i32 + 1) * hand.bid);

    return total_score as i32;
}
