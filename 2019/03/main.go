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
	seen := make(map[point]int)
	closestDistance, closestSteps := math.MaxInt64, math.MaxInt64

	x, y, steps := 0, 0, 0
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
			steps++
			seen[point{x: x, y: y}] = steps
		}
	}

	x, y, steps = 0, 0, 0
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
			steps++
			if otherSteps, present := seen[point{x: x, y: y}]; present {
				manhattan := int(math.Abs(float64(x)) + math.Abs(float64(y)))
				if manhattan < closestDistance {
					closestDistance = manhattan
				}
				totalSteps := steps + otherSteps
				if totalSteps < closestSteps {
					closestSteps = totalSteps
				}
			}
		}
	}
	println("distance: ", closestDistance)
	println("steps: ", closestSteps)
}
