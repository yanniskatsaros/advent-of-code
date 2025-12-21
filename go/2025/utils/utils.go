package utils

import (
	"fmt"
	"os"
	"strconv"
)

func Mod(a, b int) int {
	return ((a % b) + b) % b
}

func ReadInput(path string) (string, error) {
	content, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}

	return string(content), nil
}

func Sandbox() {
	s := "L86"

	s0 := string(s[0])
	fmt.Println(s0)

	i, err := strconv.Atoi(s[1:])
	if err != nil {
		fmt.Printf("%v\n", err)
		return
	}

	fmt.Println(i)
}
