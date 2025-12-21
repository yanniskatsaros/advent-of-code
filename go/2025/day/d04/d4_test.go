package d04

import (
	"fmt"
	"reflect"
	"testing"
)

func TestParseGridDiagram(t *testing.T) {
	tests := []struct {
		input   string
		want    map[Coord]GridItem
		wantErr bool
	}{
		{input: ".@.\n@@@\n...", want: map[Coord]GridItem{
			Coord{x: 0, y: 0}: Empty,
			Coord{x: 0, y: 1}: Roll,
			Coord{x: 0, y: 2}: Empty,
			Coord{x: 1, y: 0}: Roll,
			Coord{x: 1, y: 1}: Roll,
			Coord{x: 1, y: 2}: Roll,
			Coord{x: 2, y: 0}: Empty,
			Coord{x: 2, y: 1}: Empty,
			Coord{x: 2, y: 2}: Empty,
		}, wantErr: false},
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
