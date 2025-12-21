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

func TestBestBankSlice(t *testing.T) {
	t.Run("test", func(t *testing.T) {
		bank := "8119"
		max := 119

		newBank, newMax := BestBankSlice(bank, max)

		if newBank != "819" {
			t.Errorf("got = %v, want = %v", newBank, "819")
		}

		if newMax != 819 {
			t.Errorf("got = %d, want = %d", newMax, 819)
		}
	})
}

func TestMaxJoltageK(t *testing.T) {
	k := 12

	tests := []struct {
		input   string
		want    int
		wantErr bool
	}{
		{input: "987654321111111", want: 987654321111, wantErr: false},
		{input: "811111111111119", want: 811111111119, wantErr: false},
		{input: "234234234234278", want: 434234234278, wantErr: false},
		{input: "818181911112111", want: 888911112111, wantErr: false},
	}

	for i, tc := range tests {
		name := fmt.Sprintf("test %d MaxJoltageK (k=%d)", i, k)

		t.Run(name, func(t *testing.T) {
			got, err := MaxJoltageK(tc.input, k)

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
