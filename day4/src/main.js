class Card {
    constructor(cardNumber, ownNumbers, winningNumbers) {
        this.cardNumber = cardNumber;
        this.ownNumbers = ownNumbers;
        this.winningNumbers = winningNumbers;
    }
}

function parseInput(inputStr) {
    const cards = [];
    const lines = inputStr.trim().split("\n");

    lines.forEach(line => {
        const parts = line.split(": ");
        const cardNumber = parseInt(parts[0].split(" ")[1]);

        const numbers = parts[1].split(" | ");
        const winningNumbers = numbers[0].split(" ").map(n => parseInt(n));
        const ownNumbers = numbers[1].split(" ").map(n => parseInt(n));

        cards.push(new Card(cardNumber, ownNumbers, winningNumbers));
    });

    return cards;
}

function getCardScore(card) {
    let score = 0;
    card.ownNumbers.forEach(ownNumber => {
        card.winningNumbers.forEach(winningNumber => {
            if (ownNumber === winningNumber) {
                score += 1;
            }
        });
    });
    return score;
}

function solve(inputStr) {
    const cards = parseInput(inputStr);
    const counts = new Array(cards.length).fill(1);

    cards.forEach((card, i) => {
        const score = getCardScore(card);
        if (score > 0) {
            for (let j = i + 1; j < Math.min(i + 1 + score, cards.length); j++) {
                counts[j] += counts[i];
            }
        }
    });

    return counts.reduce((a, b) => a + b, 0);
}

// Main function (async to allow reading file)
async function main() {
    const fs = require('fs').promises;

    try {
        const inputStr = await fs.readFile("input.txt", "utf8");

        const start = process.hrtime.bigint();
        const result = solve(inputStr);
        console.log(`\nResult: ${result}`);

        const end = process.hrtime.bigint();
        console.log(`Time taken: ${(end - start) / BigInt(1000)} µs`);
    } catch (error) {
        console.error("Error reading file:", error);
    }
}

main();
