package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"os/exec"
	"strconv"
	"strings"
	"time"
)

// https://adventofcode.com/2019/day/15

type Op struct {
	code, params int
}

func (op Op) String() string {
	return fmt.Sprintf("Op{code: %d, params: %d}", op.code, op.params)
}

var OpAdd Op = Op{code: 1, params: 3}
var OpMlt Op = Op{code: 2, params: 3}
var OpInp Op = Op{code: 3, params: 1}
var OpOut Op = Op{code: 4, params: 1}
var OpIft Op = Op{code: 5, params: 2}
var OpIff Op = Op{code: 6, params: 2}
var OpLss Op = Op{code: 7, params: 3}
var OpEql Op = Op{code: 8, params: 3}
var OpRlo Op = Op{code: 9, params: 1}
var OpHlt Op = Op{code: 99, params: 0}

const (
	ModePos        = 0
	ModeImm        = 1
	ModeRel        = 2
	DirNone        = 0
	DirNorth       = 1
	DirSouth       = 2
	DirWest        = 3
	DirEast        = 4
	TileUnexplored = 0
	TileWall       = 1
	TileEmpty      = 2
	TileFound      = 3
	TileOxygen     = 4
)

var DirInverses = map[int]int{
	DirNorth: DirSouth,
	DirEast:  DirWest,
	DirSouth: DirNorth,
	DirWest:  DirEast,
	DirNone:  DirNone,
}

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func parseOp(opInt int) (Op, []int) {
	code := opInt % 100
	opInt /= 100 // shave off code
	var op Op
	switch code {
	case OpAdd.code:
		op = OpAdd
	case OpMlt.code:
		op = OpMlt
	case OpInp.code:
		op = OpInp
	case OpOut.code:
		op = OpOut
	case OpIft.code:
		op = OpIft
	case OpIff.code:
		op = OpIff
	case OpLss.code:
		op = OpLss
	case OpEql.code:
		op = OpEql
	case OpRlo.code:
		op = OpRlo
	case OpHlt.code:
		op = OpHlt
	default:
		panic(fmt.Sprintf("Unexpected Op code: %d", code))
	}
	modes := make([]int, op.params)
	for i := 0; i < len(modes); i++ {
		modes[i] = opInt % 10
		opInt /= 10
	}
	return op, modes
}

func getParams(prog map[int]int, offset int, modes []int, relBase int) []int {
	params := make([]int, len(modes))
	for i := 0; i < len(params); i++ {
		mode := modes[i]
		switch mode {
		case ModePos:
			params[i] = prog[offset+i]
		case ModeImm:
			params[i] = offset + i
		case ModeRel:
			params[i] = prog[offset+i] + relBase
		default:
			panic(fmt.Sprintf("Unexpected param mode: %d", mode))
		}
	}
	return params
}

func compute(prog map[int]int, input, output chan int) {
	ptr, relBase := 0, 0
	for {
		op, modes := parseOp(prog[ptr])
		if op.code == OpHlt.code {
			close(output)
			return
		}
		params := getParams(prog, ptr+1, modes, relBase)
		switch op.code {
		case OpAdd.code:
			prog[params[2]] = prog[params[0]] + prog[params[1]]
		case OpMlt.code:
			prog[params[2]] = prog[params[0]] * prog[params[1]]
		case OpInp.code:
			prog[params[0]] = <-input
		case OpOut.code:
			output <- prog[params[0]]
		case OpIft.code:
			if prog[params[0]] != 0 {
				ptr = prog[params[1]]
				continue
			}
		case OpIff.code:
			if prog[params[0]] == 0 {
				ptr = prog[params[1]]
				continue
			}
		case OpLss.code:
			if prog[params[0]] < prog[params[1]] {
				prog[params[2]] = 1
			} else {
				prog[params[2]] = 0
			}
		case OpEql.code:
			if prog[params[0]] == prog[params[1]] {
				prog[params[2]] = 1
			} else {
				prog[params[2]] = 0
			}
		case OpRlo.code:
			relBase += prog[params[0]]
		default:
			panic(fmt.Sprintf("Unexpected OpCode: %d", op.code))
		}
		ptr += op.params + 1
	}
}

type Point struct {
	x, y int
}

func clear() {
	cmd := exec.Command("clear")
	cmd.Stdout = os.Stdout
	cmd.Run()
}

func printCanvas(canvas map[Point]int, pos Point, log []int, minSteps int) {
	x_min, y_min, x_max, y_max := 0, 0, 0, 0
	for p := range canvas {
		if p.x < x_min {
			x_min = p.x
		}
		if p.y < y_min {
			y_min = p.y
		}
		if p.x > x_max {
			x_max = p.x
		}
		if p.y > y_max {
			y_max = p.y
		}
	}
	for i := x_min; i <= x_max; i++ {
		for j := y_min; j <= y_max; j++ {
			if i == pos.x && j == pos.y {
				fmt.Printf("•")
				continue
			} else if i == 0 && j == 0 {
				fmt.Printf("✚")
				continue
			}
			switch tile := canvas[Point{i, j}]; tile {
			case TileUnexplored:
				fmt.Printf("X")
			case TileEmpty:
				fmt.Printf(" ")
			case TileWall:
				fmt.Printf("█")
			case TileFound:
				fmt.Printf("✓")
			case TileOxygen:
				fmt.Printf("O")
			default:
				panic(fmt.Sprintf("Invalid tile: %d", tile))
			}
		}
		fmt.Printf("\n")
	}

	fmt.Println("pos:", pos, "steps:", len(log), "minSteps:", minSteps)
}

func move(pos Point, dir int) Point {
	switch dir {
	case DirNone:
		// pass
	case DirNorth:
		pos.y++
	case DirSouth:
		pos.y--
	case DirEast:
		pos.x++
	case DirWest:
		pos.x--
	default:
		panic(fmt.Sprintf("Invalid direction: %d", dir))
	}
	return pos
}

func exploreNeighbor(canvas map[Point]int, pos Point) int {
	if canvas[Point{pos.x, pos.y + 1}] == TileUnexplored {
		return DirNorth
	}
	if canvas[Point{pos.x + 1, pos.y}] == TileUnexplored {
		return DirEast
	}
	if canvas[Point{pos.x, pos.y - 1}] == TileUnexplored {
		return DirSouth
	}
	if canvas[Point{pos.x - 1, pos.y}] == TileUnexplored {
		return DirWest
	}
	return DirNone
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)
	ss := strings.Split(string(file), ",")

	// use a map to allow prog to exand itself, write past initial size
	prog := make(map[int]int, len(ss))
	for i, s := range ss {
		num, err := strconv.Atoi(strings.Trim(s, "\n "))
		check(err)
		prog[i] = num
	}

	input := make(chan int)
	output := make(chan int)
	canvas := make(map[Point]int)

	var goal Point
	minSteps := math.MaxInt32
	log := make([]int, 0)

	pos := Point{0, 0}
	dir := exploreNeighbor(canvas, pos)
	newPos := move(pos, dir)
	count := 1

	go compute(prog, input, output)
	input <- dir
	for result := range output {
		canvas[newPos] = result + 1 // +1 to allow for 0 as defualt canvas val
		switch result + 1 {
		case TileWall:
			// pass
		case TileEmpty:
			log = append(log, dir)
			pos = newPos
			count++
		case TileFound:
			log = append(log, dir)
			pos = newPos
			count++
			goal = pos
			if len(log) < minSteps {
				minSteps = len(log)
			}
		default:
			panic(fmt.Sprintf("Invalid result: %d", result))
		}

		//clear()
		//printCanvas(canvas, pos, log, minSteps)
		//time.Sleep(100 * time.Millisecond)

		dir = exploreNeighbor(canvas, pos)
		if pos.x == 0 && pos.y == 0 && dir == DirNone {
			break
		}
		if dir == DirNone || result+1 == TileFound {
			for len(log) > 0 {
				dir = exploreNeighbor(canvas, pos)
				if dir != DirNone {
					break
				}
				prev := log[len(log)-1]
				log = log[:len(log)-1] // "pop the stack"
				newPos = move(pos, DirInverses[prev])
				input <- DirInverses[prev]
				result = <-output
				if result+1 != TileEmpty {
					panic(fmt.Sprintf("Backtracking into non-empty tile: %d", result+1))
				}
				pos = newPos

				//clear()
				//printCanvas(canvas, pos, log, minSteps)
				//time.Sleep(100 * time.Millisecond)
			}
		}

		input <- dir
		newPos = move(pos, dir)
	}
	printCanvas(canvas, pos, log, minSteps)
	time.Sleep(100 * time.Millisecond)

	steps := -1 // initial oxygenation doesn't count as a step
	workQueue := []Point{goal}
	for len(workQueue) > 0 {
		next := make([]Point, 0)
		for _, curr := range workQueue {
			canvas[curr] = TileOxygen
			if canvas[Point{curr.x, curr.y + 1}] == TileEmpty {
				next = append(next, Point{curr.x, curr.y + 1})
			}
			if canvas[Point{curr.x + 1, curr.y}] == TileEmpty {
				next = append(next, Point{curr.x + 1, curr.y})
			}
			if canvas[Point{curr.x, curr.y - 1}] == TileEmpty {
				next = append(next, Point{curr.x, curr.y - 1})
			}
			if canvas[Point{curr.x - 1, curr.y}] == TileEmpty {
				next = append(next, Point{curr.x - 1, curr.y})
			}
		}
		workQueue = next
		steps++

		//clear()
		//printCanvas(canvas, pos, log, minSteps)
		//time.Sleep(100 * time.Millisecond)
	}
	fmt.Println("steps:", steps)
}
