package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strconv"
	"strings"
)

// https://adventofcode.com/2019/day/3

func check(err error) {
	if err != nil {
		panic(err)
	}
}

type point struct {
	x, y int
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)

	ss := strings.Fields(string(file))
	frst := strings.Split(ss[0], ",")
	scnd := strings.Split(ss[1], ",")
	seen := make(map[point]bool)
	closest := math.MaxInt64

	x, y := 0, 0
	for _, s := range frst {
		n, err := strconv.Atoi(s[1:])
		check(err)
		var step int
		var ord *int
		vec := s[0:1]
		switch vec {
		case "U":
			step = 1
			ord = &y
		case "D":
			step = -1
			ord = &y
		case "R":
			step = 1
			ord = &x
		case "L":
			step = -1
			ord = &x
		default:
			panic(fmt.Sprintf("Invalid vec: %s", vec))
		}
		for i := 0; i < n; i++ {
			*ord += step
			seen[point{x: x, y: y}] = true
		}
	}

	x, y = 0, 0
	for _, s := range scnd {
		n, err := strconv.Atoi(s[1:])
		check(err)
		var step int
		var ord *int
		vec := s[0:1]
		switch vec {
		case "U":
			step = 1
			ord = &y
		case "D":
			step = -1
			ord = &y
		case "R":
			step = 1
			ord = &x
		case "L":
			step = -1
			ord = &x
		default:
			panic(fmt.Sprintf("Invalid vec: %s", vec))
		}
		for i := 0; i < n; i++ {
			*ord += step
			if _, present := seen[point{x: x, y: y}]; present {
				distance := int(math.Abs(float64(x)) + math.Abs(float64(y)))
				if distance < closest {
					closest = distance
				}
			}
		}
	}
	println(closest)
}
