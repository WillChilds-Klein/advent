package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"strconv"
	"strings"
)

// https://adventofcode.com/2019/day/13

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
	ModePos      = 0
	ModeImm      = 1
	ModeRel      = 2
	TileEmpty    = 0
	TileWall     = 1
	TileBlock    = 2
	TilePaddle   = 3
	TileBall     = 4
	StickLeft    = -1
	StickNeutral = 0
	StickRight   = 1
)

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

func printCanvas(canvas map[Point]int, score int) {
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
			switch canvas[Point{i, j}] {
			case TileEmpty:
				fmt.Printf(" ")
			case TileWall:
				fmt.Printf("|")
			case TileBlock:
				fmt.Printf("█")
			case TilePaddle:
				fmt.Printf("▔")
			case TileBall:
				fmt.Printf("•")
			}
		}
		fmt.Printf("\n")
	}
	fmt.Println("SCORE:", score)
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

	// then, "insert quarters" and play
	score := -1
	ballPos, paddlePos := Point{-1, -1}, Point{-1, -1}
	stick := 0
	input := make(chan int)
	output := make(chan int)
	canvas := make(map[Point]int)
	prog[0] = 2
	go compute(prog, input, output)
	for {
		if ballPos.y < paddlePos.y {
			stick = -1
		} else if ballPos.y > paddlePos.y {
			stick = 1
		} else {
			stick = 0
		}
		select {
		case input <- stick:
		case x := <-output:
			y := <-output
			third := <-output
			if x == -1 && y == 0 {
				score = third
			} else {
				canvas[Point{y, x}] = third
				if third == TileBall {
					ballPos = Point{y, x}
				} else if third == TilePaddle {
					paddlePos = Point{y, x}
				}
			}
			fmt.Println(x, y, third)
			printCanvas(canvas, score)
			clear()
		default:
			break
		}
	}
	printCanvas(canvas, score)
}
