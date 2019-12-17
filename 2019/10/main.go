package main

import (
	"fmt"
	"io/ioUtil"
	"math"
	"os"
	"strings"
)

// https://adventofcode.com/2019/day/10

func check(err error) {
	if err != nil {
		panic(err)
	}
}

type Point struct {
	x, y int
}

// TODO: make this less trash, find better algorithm for gcd
func gcd(x, y float64) float64 {
	max := 1.0
	for i := 2; i <= int(math.Min(math.Abs(x), math.Abs(y))); i++ {
		if int(x)%i == 0 && int(y)%i == 0 {
			max = float64(i)
		}
	}
	return max
}

func step(p, q Point) (int, int) {
	x_diff, y_diff := q.x-p.x, q.y-p.y
	if x_diff == 0 {
		if p.y > q.y {
			return 0, -1
		}
		return 0, 1
	}
	if y_diff == 0 {
		if p.x > q.x {
			return -1, 0
		}
		return 1, 0
	}
	gcf := int(math.Abs(gcd(float64(x_diff), float64(y_diff))))
	return x_diff / gcf, y_diff / gcf
}

func visible(universe map[Point]bool, p, q Point) bool {
	x_step, y_step := step(p, q)
	for x, y := p.x+x_step, p.y+y_step; x != q.x || y != q.y; x, y = x+x_step, y+y_step {
		if (universe[Point{x, y}]) {
			return false
		}
	}
	return true
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)

	rows := strings.Split(string(file), "\n")
	universe := make(map[Point]bool, 0)
	for i, row := range rows {
		for j, itm := range strings.Split(row, "") {
			switch itm {
			case "#":
				universe[Point{x: i, y: j}] = true
			case ".":
				// no-op
			default:
				panic(fmt.Sprintf("Invalid universe member: %s", itm))
			}
		}
	}

	max := 0
	for asteroid, _ := range universe {
		count := 0
		for other, _ := range universe {
			if asteroid == other {
				continue
			}
			if visible(universe, asteroid, other) {
				count++
			}
		}
		if count > max {
			max = count
		}
	}
	println(max)
}
