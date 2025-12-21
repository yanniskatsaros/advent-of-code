package d03

import (
	"fmt"
	"strconv"
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
			joltage := digit*10 + maxSecondDigit

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

func BestBankSlice(bank string, max int) (string, int) {
	n := len(bank)

	newBank := bank
	newMax := max

	// TODO: add note / explanation why given input with len = n, we return n - 1

	for i := range n {
		// determine if removing the digit at this index increases the max
		bankSlice := bank[:i] + bank[i+1:]

		// TODO: handle error
		candidate, _ := strconv.Atoi(bankSlice)
		// fmt.Printf("(i = %d) candidate = %d\n", i, candidate)

		if candidate >= newMax {
			newMax = candidate
			newBank = bankSlice
		} else {
			// we have reached a point where we cannot "push" the index
			// any further to the right of the bank
			break
		}
	}

	return newBank, newMax
}

func MaxJoltageK(bank string, k int) int {
	n := len(bank)

	// TODO: lazy error handling for now
	if n <= k {
		return -1
	}

	max := -1
	offset := n - k
	bankslice := bank[offset:]

	for i := range n - k {
		offset = n - k - i - 1 // 3, 2, 1, 0 etc.

		bankslice = string(bank[offset]) + bankslice
		// fmt.Printf("===  (n = %d, k = %d, offset = %d)  ===\nin = %v\n", n, k, offset, bankslice)

		bankslice, max = BestBankSlice(bankslice, max)
		// fmt.Printf("out = %v, max = %d\n", bankslice, max)
	}

	return max
}

func Part1() {
	input, err := utils.ReadInput("inputs/day3.txt")
	if err != nil {
		fmt.Printf("%v", err)
		return
	}
	input = strings.TrimSpace(input)
	lines := strings.Split(input, "\n")

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
	}

	fmt.Printf("Total Output Joltage: %d\n", totalJoltage)
}

func Part2() {
	// bank := "987654321111111"
	// bank := "811111111111119"
	// bank := "234234234234278"
	// bank := "818181911112111"

	input, err := utils.ReadInput("inputs/day3.txt")
	if err != nil {
		fmt.Printf("%v", err)
		return
	}
	input = strings.TrimSpace(input)
	lines := strings.Split(input, "\n")

	k := 12
	totalJoltage := 0

	for _, bank := range lines {
		bank = strings.TrimSpace(bank)
		joltage := MaxJoltageK(bank, k)

		// if err != nil {
		// 	fmt.Errorf("Failed to find max joltage: %v", err)
		// 	return
		// }

		fmt.Printf("Bank = %v (max = %d)\n", bank, joltage)

		totalJoltage += joltage
	}

	fmt.Printf("Total Output Joltage: %d\n", totalJoltage)
}
