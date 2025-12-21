package d04

import (
	"fmt"
	"strings"
)

type Coord struct {
	x int
	y int
}

type GridItem int

const (
	Empty GridItem = iota
	Roll
)

func ParseGridDiagram(diagram string) (map[Coord]GridItem, error) {
	grid := make(map[Coord]GridItem)

	lines := strings.Split(strings.TrimSpace(diagram), "\n")

	for i, line := range lines {
		line = strings.TrimSpace(line)

		for j, c := range line {
			var item GridItem

			switch string(c) {
			case "@":
				item = Roll

			case ".":
				item = Empty

			default:
				return nil, fmt.Errorf("Found unknown character when parsing grid: %v", string(c))
			}

			coord := Coord{x: i, y: j}
			grid[coord] = item

			// fmt.Printf("(i, j) = (%d, %d) item = %v\n", i, j, item)
		}
	}

	return grid, nil
}

func Part1() {
	diagram := `
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
	`

	grid, err := ParseGridDiagram(diagram)
	if err != nil {
		fmt.Printf("%v\n", err)
		return
	}

	fmt.Printf("grid = %v", grid)
}
