package d03

import (
	"fmt"
	"testing"
)

func TestMaxJoltage(t *testing.T) {
	tests := []struct {
		input   string
		want    int
		wantErr bool
	}{
		{input: "987654321111111", want: 98, wantErr: false},
		{input: "811111111111119", want: 89, wantErr: false},
		{input: "234234234234278", want: 78, wantErr: false},
		{input: "818181911112111", want: 92, wantErr: false},
	}

	for i, tc := range tests {
		name := fmt.Sprintf("test %d MaxJoltage", i)

		t.Run(name, func(t *testing.T) {
			got, err := MaxJoltage(tc.input)

			if (err != nil) != tc.wantErr {
				t.Errorf("error = %v, wantErr = %v", err, tc.wantErr)
				return
			}

			if got != tc.want {
				t.Errorf("input = %v, want = %v, got = %v", tc.input, tc.want, got)
			}
		})
	}
}
