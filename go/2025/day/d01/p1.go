package d01

import (
	"fmt"
	"errors"
	"strconv"
	"strings"

	"yanniskatsaros/aoc/2025/utils"
)


type DialPosition int


func applyRotation(instr string, current DialPosition) (DialPosition, error) {
	// instr examples: L68, R60 etc.
	// direction: L or R

	invalidInstructionErr := func(instr string) error {
		return errors.New(fmt.Sprintf("Invalid instruction: %v", instr))
	}

	runes := []rune(instr)

	if len(runes) == 0 {
		return current, errors.New(fmt.Sprintf("Empty instruction: %v", instr))
	}

	direction := string(runes[0])
	rest := string(runes[1:])

	if direction == "L" {
		i, err := strconv.Atoi(rest)
		if err != nil {
			return current, invalidInstructionErr(instr)
		}

		new := utils.Mod(int(current) - i, 100)
		return DialPosition(new), nil
	}

	if direction == "R" {
		i, err := strconv.Atoi(rest)
		if err != nil {
			return current, invalidInstructionErr(instr)
		}

		new := utils.Mod(int(current) + i, 100)
		return DialPosition(new), nil
	}

	// if neither L or R this is an invalid instruction
	return current, errors.New(fmt.Sprintf("Invalid instruction direction: %v", instr))
}


func Part1() {
	var dial DialPosition = 50
	fmt.Printf("Dial: %v\n", dial)

	input, err := utils.ReadInput("inputs/day1.txt")
	if err != nil {
		fmt.Printf("%v", err)
		return
	}
	lines := strings.Split(input, "\n")

	// the password is the number of times the dial is left at the 0 position
	var counter int = 0

	for _, line := range lines {
		line = strings.TrimSpace(line)
		
		dial, err = applyRotation(line, dial)

		if int(dial) == 0 {
			counter += 1
		}
	}

	fmt.Printf("Password: %v\n", counter)
}
