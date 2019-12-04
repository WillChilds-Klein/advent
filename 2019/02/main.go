package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

// https://adventofcode.com/2019/day/2

const (
	StepSize = 4
	OpAdd    = 1
	OpMult   = 2
	OpHalt   = 99
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func compute(prog []int) {
	for i := 0; prog[i] != OpHalt; i += StepSize {
		switch prog[i] {
		case OpAdd:
			prog[prog[i+3]] = prog[prog[i+1]] + prog[prog[i+2]]
		case OpMult:
			prog[prog[i+3]] = prog[prog[i+1]] * prog[prog[i+2]]
		default:
			panic(fmt.Sprintf("Unexpected OpCode: %d", prog[i]))
		}
	}
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)
	ss := strings.Split(string(file), ",")
	prog := make([]int, len(ss))
	for i, s := range ss {
		num, err := strconv.Atoi(s)
		check(err)
		prog[i] = num
	}
	prog[1] = 12
	prog[2] = 2
	compute(prog)
	fmt.Printf("%v", prog[0])
}
