package d04

import (
	"fmt"
	"reflect"
	"testing"
)

func TestParseGridDiagram(t *testing.T) {
	tests := []struct {
		input   string
		want    Grid
		wantErr bool
	}{
		{
			input: ".@.\n@@@\n...", want: Grid{
				grid: map[Coord]GridItem{
					Coord{x: 0, y: 0}: Empty,
					Coord{x: 0, y: 1}: Roll,
					Coord{x: 0, y: 2}: Empty,
					Coord{x: 1, y: 0}: Roll,
					Coord{x: 1, y: 1}: Roll,
					Coord{x: 1, y: 2}: Roll,
					Coord{x: 2, y: 0}: Empty,
					Coord{x: 2, y: 1}: Empty,
					Coord{x: 2, y: 2}: Empty,
				},
				rows: 3,
				cols: 3,
			},
			wantErr: false,
		},
	}

	for i, tc := range tests {
		name := fmt.Sprintf("test %d ParseGridDiagram", i)

		t.Run(name, func(t *testing.T) {
			got, err := ParseGridDiagram(tc.input)

			if (err != nil) != tc.wantErr {
				t.Errorf("error = %v, wantErr = %v", err, tc.wantErr)
				return
			}

			if !reflect.DeepEqual(got, tc.want) {
				t.Errorf("input = %v, want = %v, got = %v", tc.input, tc.want, got)
			}
		})
	}
}

func TestGridWithinBounds(t *testing.T) {
	grid := Grid{
		grid: map[Coord]GridItem{
			Coord{x: 0, y: 0}: Empty,
			Coord{x: 0, y: 1}: Roll,
			Coord{x: 0, y: 2}: Empty,
			Coord{x: 1, y: 0}: Roll,
			Coord{x: 1, y: 1}: Roll,
			Coord{x: 1, y: 2}: Roll,
			Coord{x: 2, y: 0}: Empty,
			Coord{x: 2, y: 1}: Empty,
			Coord{x: 2, y: 2}: Empty,
		},
		rows: 3,
		cols: 3,
	}

	tests := []struct {
		input Coord
		want  bool
	}{
		{input: Coord{x: 0, y: 0}, want: true},
		{input: Coord{x: 0, y: 2}, want: true},
		{input: Coord{x: 2, y: 2}, want: true},
		{input: Coord{x: -1, y: 0}, want: false},
		{input: Coord{x: -1, y: -1}, want: false},
		{input: Coord{x: 0, y: -1}, want: false},
		{input: Coord{x: 3, y: 0}, want: false},
	}

	for i, tc := range tests {
		name := fmt.Sprintf("test %d", i)

		t.Run(name, func(t *testing.T) {
			got := grid.WithinBounds(tc.input)

			if got != tc.want {
				t.Errorf("input = %v, want = %v, got = %v", tc.input, tc.want, got)
			}
		})
	}
}
