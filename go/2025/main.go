package main

import (
	"fmt"
	"os"
	"yanniskatsaros/aoc/2025/day/d01"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Usage: go run . <day> <part>")
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
