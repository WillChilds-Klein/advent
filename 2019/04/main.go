package main

import (
	"io/ioutil"
	"math"
	"os"
	"strconv"
	"strings"
)

// https://adventofcode.com/2019/day/4

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
	if len(ss) != 1 {
		panic("Invalid input: need 1 line")
	}
	bounds := strings.Split(ss[0], "-")
	if len(bounds) != 2 {
		panic("Invalid input: need 2 bounds")
	}

	start, err := strconv.Atoi(bounds[0])
	check(err)
	end, err := strconv.Atoi(bounds[1])
	check(err)
	count := 0

	for i := start; i <= end; i++ {
		if i/10000 < 1 {
			continue
		}
		hasInc := true
		doubleCount, runLen := 0, 0
		// iterate over digits from right to left
		prev := math.MaxInt64
		for base := i; base != 0; base /= 10 {
			rmdr := base % 10
			if rmdr > prev {
				hasInc = false
			}
			if rmdr == prev {
				runLen++
				if runLen == 1 {
					doubleCount++
				} else if runLen == 2 {
					doubleCount--
				}
			} else {
				runLen = 0
			}
			prev = rmdr
		}
		if doubleCount < 1 || !hasInc {
			continue
		}
		count++
	}
	println(count)
}
