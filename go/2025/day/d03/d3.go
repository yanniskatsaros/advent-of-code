package d03

import (
	"fmt"
	// "strconv"
	"strings"
	"yanniskatsaros/aoc/2025/utils"
)

func MaxJoltage(bank string) (int, error) {
	n := len(bank)

	if n < 2 {
		return -1, fmt.Errorf("Cannot find max joltage for less than 2 digits: %v", bank)
	}

	maxJoltage := 0

	// the largest **second** digit seen during iteration from right -> left
	maxSecondDigit := -1

	// scan from right to left 
	for i := n - 1; i >= 0; i-- {
		digit := int(bank[i] - '0')

		// if we have seen a digit to the right, we can form a pair:
		if maxSecondDigit != -1 {
			joltage := digit * 10 + maxSecondDigit

			if joltage > maxJoltage {
				maxJoltage = joltage
			}
		}

		// save the max digit to the right seen thus far:
		if digit > maxSecondDigit {
			maxSecondDigit = digit
		}
	}

	return maxJoltage, nil
}

func Part1() {
	input, err := utils.ReadInput("inputs/day3.txt")
	if err != nil {
		fmt.Printf("%v", err)
		return
	}
	input = strings.TrimSpace(input)
	lines := strings.Split(input, "\n")

	// // inline testing
	// lines = []string{
	// 	"987654321111111",
	// 	"811111111111119",
	// 	"234234234234278",
	// 	"818181911112111",
	// 	"281",
	// 	"2918",
	// 	"918",
	// }

	totalJoltage := 0

	for _, bank := range lines {
		bank = strings.TrimSpace(bank)
		joltage, err := MaxJoltage(bank)

		if err != nil {
			fmt.Errorf("Failed to find max joltage: %v", err)
			return
		}

		fmt.Printf("Bank = %v (max = %d)\n", bank, joltage)

		totalJoltage += joltage

		// fmt.Printf("Max Joltage: %d\n", joltage)
	}

	fmt.Printf("Total Output Joltage: %d\n", totalJoltage)
}
