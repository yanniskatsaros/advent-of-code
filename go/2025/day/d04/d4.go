package d04

import (
	"fmt"
	"iter"
	"strings"

	"yanniskatsaros/aoc/2025/utils"
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

type Grid struct {
	grid map[Coord]GridItem
	rows int
	cols int
}

func EmptyGrid() Grid {
	grid := make(map[Coord]GridItem)
	return Grid{grid: grid, rows: 0, cols: 0}
}

func (g Grid) Items() iter.Seq2[Coord, GridItem] {
	return func(yield func(Coord, GridItem) bool) {
		// iterate this way to retain (x, y) grid ordering
		for i := range g.rows {
			for j := range g.cols {
				coord := Coord{x: i, y: j}
				item := g.grid[coord]

				if !yield(coord, item) {
					return
				}
			}
		}
	}
}

func (g Grid) WithinBounds(coord Coord) bool {
	return (coord.x >= 0) &&
		(coord.x < g.rows) &&
		(coord.y >= 0) &&
		(coord.y < g.cols)
}

func ParseGridDiagram(diagram string) (Grid, error) {
	grid := make(map[Coord]GridItem)

	lines := strings.Split(strings.TrimSpace(diagram), "\n")

	// track the total number of rows and columns in the grid
	rows := len(lines)
	cols := 0

	for i, line := range lines {
		line = strings.TrimSpace(line)
		cols = len(line)

		for j, c := range line {
			var item GridItem

			switch string(c) {
			case "@":
				item = Roll

			case ".":
				item = Empty

			default:
				return EmptyGrid(), fmt.Errorf("Found unknown character when parsing grid: %v", string(c))
			}

			coord := Coord{x: i, y: j}
			grid[coord] = item

			// fmt.Printf("(i, j) = (%d, %d) item = %v\n", i, j, item)
		}
	}

	return Grid{grid: grid, rows: rows, cols: cols}, nil
}

func CountRolls(items map[Coord]GridItem) int {
	count := 0

	for _, item := range items {
		if item == Roll {
			count++
		}
	}

	return count
}

func (g Grid) AdjacentItems(coord Coord) map[Coord]GridItem {
	adj := make(map[Coord]GridItem)

	// We need to check 8 total positions surrounding the given `coord`
	//
	// 123
	// 8@4
	// 765
	//
	// NOTE: we assume **top left** is the (0, 0) coordinate position

	relpos := []int{-1, 0, 1}

	for _, i := range relpos {
		for _, j := range relpos {
			if (i == 0) && (j == 0) {
				// the current index of `coord` is not considered adjacent
				continue
			}

			c := Coord{x: coord.x + i, y: coord.y + j}
			if g.WithinBounds(c) {
				adj[c] = g.grid[c]
			}
		}
	}

	return adj
}

func RemoveRolls(g Grid) (Grid, int) {
	remove := []Coord{}

	for coord, item := range g.Items() {
		adj := g.AdjacentItems(coord)
		count := CountRolls(adj)

		if (item == Roll) && (count < 4) {
			remove = append(remove, coord)
		}
	}

	grid := g.grid

	for _, c := range remove {
		grid[c] = Empty
	}

	return Grid{grid: grid, rows: g.rows, cols: g.cols}, len(remove)
}

func Part1() {
	diagram, err := utils.ReadInput("inputs/day4.txt")
	if err != nil {
		fmt.Printf("%v\n", err)
		return
	}

	diagram = strings.TrimSpace(diagram)
	grid, err := ParseGridDiagram(diagram)
	if err != nil {
		fmt.Printf("%v\n", err)
		return
	}

	// fmt.Printf("grid = %v", grid)

	total := 0
	for coord, item := range grid.Items() {
		adj := grid.AdjacentItems(coord)
		count := CountRolls(adj)

		if (item == Roll) && (count < 4) {
			total++
			// fmt.Printf("coord = %v\n", coord)
		}
	}

	fmt.Printf("Total = %v\n", total)
}

func Part2() {
	diagram, err := utils.ReadInput("inputs/day4.txt")
	if err != nil {
		fmt.Printf("%v\n", err)
		return
	}

	diagram = strings.TrimSpace(diagram)
	grid, err := ParseGridDiagram(diagram)
	if err != nil {
		fmt.Printf("%v\n", err)
		return
	}

	total := 0
	var removed int

	for {
		grid, removed = RemoveRolls(grid)
		total += removed

		if removed == 0 {
			break
		}

		// fmt.Printf("removed = %v, total = %v\n", removed, total)
	}

	fmt.Printf("Total Removed = %v\n", total)
}
