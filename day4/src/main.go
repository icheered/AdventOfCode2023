package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

type Card struct {
	CardNumber     int
	OwnNumbers     []int
	WinningNumbers []int
}

func parseInput(inputStr string) []Card {
	var cards []Card
	lines := strings.Split(strings.TrimSpace(inputStr), "\n")

	for _, line := range lines {
		parts := strings.Split(line, ": ")
		cardNumber, _ := strconv.Atoi(strings.Split(parts[0], " ")[1])

		numbers := strings.Split(parts[1], " | ")
		winningNumbers := convertToIntSlice(strings.Split(numbers[0], " "))
		ownNumbers := convertToIntSlice(strings.Split(numbers[1], " "))

		cards = append(cards, Card{cardNumber, ownNumbers, winningNumbers})
	}

	return cards
}

func getCardScore(card Card) int {
	score := 0
	for _, ownNumber := range card.OwnNumbers {
		for _, winningNumber := range card.WinningNumbers {
			if ownNumber == winningNumber {
				score++
			}
		}
	}
	return score
}

func solve(inputStr string) int {
	cards := parseInput(inputStr)
	counts := make([]int, len(cards))
	for i := range counts {
		counts[i] = 1
	}

	for i, card := range cards {
		score := getCardScore(card)
		if score > 0 {
			for j := i + 1; j < min(i+1+score, len(cards)); j++ {
				counts[j] += counts[i]
			}
		}
	}

	sum := 0
	for _, count := range counts {
		sum += count
	}
	return sum
}

func convertToIntSlice(strSlice []string) []int {
	var intSlice []int
	for _, str := range strSlice {
		num, _ := strconv.Atoi(str)
		intSlice = append(intSlice, num)
	}
	return intSlice
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)
	inputStr := ""
	for scanner.Scan() {
		inputStr += scanner.Text() + "\n"
	}

	start := time.Now()
	result := solve(inputStr)
	fmt.Printf("\nResult: %d\n", result)

	end := time.Now()
	fmt.Printf("Time taken: %d µs\n", end.Sub(start).Microseconds())
}
