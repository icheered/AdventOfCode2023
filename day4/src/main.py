from typing import List, NamedTuple
import time

class Card(NamedTuple):
    card_number: int
    own_numbers: List[int]
    winning_numbers: List[int]

def parse_input(input_str: str) -> List[Card]:
    cards = []
    for line in input_str.strip().split("\n"):
        parts = line.split(": ")
        card_number = int(parts[0].split()[1])

        numbers = parts[1].split(" | ")
        winning_numbers = [int(n) for n in numbers[0].split()]
        own_numbers = [int(n) for n in numbers[1].split()]

        cards.append(Card(card_number, own_numbers, winning_numbers))

    return cards

def get_card_score(card: Card) -> int:
    score = 0
    for own_number in card.own_numbers:
        for winning_number in card.winning_numbers:
            if own_number == winning_number:
                score += 1
    return score

def solve(input_str: str) -> int:
    cards = parse_input(input_str)
    counts = [1] * len(cards)

    for i, card in enumerate(cards):
        score = get_card_score(card)
        if score > 0:
            for j in range(i + 1, min(i + 1 + score, len(cards))):
                counts[j] += counts[i]

    return sum(counts)

def main():
    with open("input.txt") as f:
        input_str = f.read()

    start = time.time()
    result = solve(input_str)
    # println!("\nResult: {}", result);
    print(f"\nResult: {result}")

    end = time.time()
    # Print time in micros
    print(f"Time taken: {round((end - start) * 1000 * 1000)} µs")

if __name__ == "__main__":
    main()