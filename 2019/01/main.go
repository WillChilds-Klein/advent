package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

// https://adventofcode.com/2019/day/1

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func cost(input int) int {
	return int(input/3) - 2
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)
	ss := strings.Fields(string(file))
	fuel := 0
	for _, s := range ss {
		i, err := strconv.Atoi(s)
		check(err)
		for rmdr := cost(i); rmdr > 0; rmdr = cost(rmdr) {
			fuel += rmdr
		}
	}
	fmt.Printf("%d\n", fuel)
}
