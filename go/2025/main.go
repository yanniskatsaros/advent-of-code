package main

import (
	"fmt"
	"os"
	"yanniskatsaros/aoc/2025/day/d01"
	"yanniskatsaros/aoc/2025/utils"
)

func main() {
	// check for "sandbox" mode first
	if len(os.Args) == 2 {
		arg1 := os.Args[1]
		if arg1 == "--sandbox" {
			utils.Sandbox()
			return
		}
	}

	// otherwise parse as <day> <part>
	if len(os.Args) < 3 {
		fmt.Println("Usage: go run . <day> <part>  //  or: go run . --sandbox")
		return
	}

	day := os.Args[1]
	part := os.Args[2]

	switch day + part {
	case "11":
		d01.Part1()

	default:
		fmt.Printf("Day %v Part %v not implemented yet\n", day, part)
	}
}
