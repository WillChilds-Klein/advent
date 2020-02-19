package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strconv"
	"strings"
)

// https://adventofcode.com/2019/day/16

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func doPhase(input []int) []int {
	base := [4]int{0, 1, 0, -1}
	output := make([]int, len(input))
	for i, _ := range input {
		pattern := make([]int, 0)
		for j := 0; j < len(base); j++ {
			for p := 0; p <= i; p++ {
				pattern = append(pattern, base[j%len(base)])
			}
		}
		sum := 0
		for j := 1; j < len(input)+1; j++ {
			sum += input[j-1] * pattern[j%len(pattern)]
		}
		output[i] = int(math.Abs(float64(sum % 10)))
	}
	return output
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)
	ss := strings.Split(string(file), "")
	input := make([]int, len(ss))
	for i, n := range ss {
		input[i], err = strconv.Atoi(n)
		if err != nil {
			panic("Bad int parse!")
		}
	}
	fmt.Println("INPUT:", input)
	for i := 0; i < 100; i++ {
		input = doPhase(input)
	}
	fmt.Println("OUTPUT", input)
	output := make([]string, len(input))
	for i, _ := range input {
		output[i] = strconv.Itoa(input[i])
	}
	fmt.Println("FORMATTED:", strings.Join(output, ""))
}
