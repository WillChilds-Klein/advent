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

func firstVisible(universe map[Point]bool, p, q Point) Point {
	x_step, y_step := step(p, q)
	for x, y := p.x+x_step, p.y+y_step; x != q.x || y != q.y; x, y = x+x_step, y+y_step {
		current := Point{x, y}
		if universe[current] {
			return current
		}
	}
	return q
}

func nextCoordinates(x_prev, y_prev, x_max, y_max int) (int, int) {
	x, y := x_prev, y_prev
	if x == 0 && y == 0 {
		y++
	} else if x == 0 && y == y_max {
		x++
	} else if x == x_max && y == y_max {
		y--
	} else if x == x_max && y == 0 {
		x--
	} else if x == 0 {
		y++
	} else if y == 0 {
		x--
	} else if y == y_max {
		x++
	} else if x == x_max {
		y--
	}
	return x, y
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)

	rows := strings.Split(strings.Trim(string(file), "\n"), "\n")
	universe := make(map[Point]bool, 0)
	for i, row := range rows {
		for j, itm := range strings.Split(row, "") {
			switch itm {
			case "#":
				universe[Point{i, j}] = true
			case ".":
				// no-op
			default:
				panic(fmt.Sprintf("Invalid universe member: %s", itm))
			}
		}
	}

	maxCount := 0
	var maxCoord Point
	for asteroid, _ := range universe {
		count := 0
		for other, _ := range universe {
			if asteroid == other {
				continue
			}
			visible := firstVisible(universe, asteroid, other) == other
			if visible {
				count++
			}
		}
		if count > maxCount {
			maxCount = count
			maxCoord = asteroid
		}
	}
	fmt.Println(maxCount, maxCoord)

	x_max, y_max := len(rows)-1, len(rows[0])-1
	x, y := 0, maxCoord.y-1
	var twhdthCoord Point
	destroyed := 0
	for {
		x, y = nextCoordinates(x, y, x_max, y_max)
		curr := Point{x, y}
		if curr == maxCoord {
			continue
		}
		target := firstVisible(universe, maxCoord, curr)
		if universe[target] {
			universe[target] = false
			destroyed++
			fmt.Println(target, destroyed)
		}
		if destroyed == 200 {
			twhdthCoord = target
			break
		}
	}
	fmt.Println(100*twhdthCoord.x + twhdthCoord.y)
}
