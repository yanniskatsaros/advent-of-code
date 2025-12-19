package d03

import (
	"fmt"
	"strconv"
	"strings"
	"yanniskatsaros/aoc/2025/utils"
)

func MaxJoltage(bank string) (int, error) {
	n := len(bank)
	lastIndex := n - 1

	var maxFirst int
	var maxSecond int
	var max int

	var iMaxFirst int
	var iMaxSecond int

	for i, char := range bank {
		digit := int(char - '0') // Convert rune to int
		// fmt.Printf("%v: %d (max = %d%d) (i1, i2 = %d, %d)\n", i, digit, maxFirst, maxSecond, iMaxFirst, iMaxSecond)

		// incredibly big brained code here:
		if i == 0 {
			maxFirst = digit
			iMaxFirst = i
			// fmt.Println("Skip 1")
			continue
		}

		if i == 1 {
			maxSecond = digit
			iMaxSecond = i
			// fmt.Println("Skip 2")
			continue
		}

		// NOTE: we don't update the first index if this is the last "slot"
		if (digit > maxFirst) && (i != lastIndex) {
			// fmt.Printf("Skip 3: maxFirst = %d, iMaxFirst = %d, digit = %d\n", maxFirst, iMaxFirst, digit)
			maxFirst = digit
			iMaxFirst = i

			// NOTE: we need to do another check:
			// if the index of the first max is > the index of the second max
			// we need to update the second max to the adjacent char to the right of *this* one:

			if iMaxFirst > iMaxSecond {
				iMaxSecond = i + 1 // the next index
				maxSecond = int(bank[iMaxSecond] - '0')
			}

			continue
		}

		if digit > maxSecond {
			// fmt.Printf("Skip 4: maxSecond = %d, iMaxSecond = %d, digit = %d\n", maxSecond, iMaxSecond, digit)
			maxSecond = digit
			iMaxSecond = i
			continue
		}
	}

	var err error
	max, err = strconv.Atoi(fmt.Sprintf("%d%d", maxFirst, maxSecond))
	if err != nil {
		return -1, err
	}

	return max, nil
}

func Part1() {
	input, err := utils.ReadInput("inputs/day3.txt")
	if err != nil {
		fmt.Printf("%v", err)
		return
	}
	input = strings.TrimSpace(input)
	lines := strings.Split(input, "\n")

	// inline testing
	lines = []string{
		"987654321111111",
		"811111111111119",
		"234234234234278",
		"818181911112111",
	}

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
