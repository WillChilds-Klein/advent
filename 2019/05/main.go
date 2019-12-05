package main

import (
	"bufio"
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

// https://adventofcode.com/2019/day/2

type Op struct {
	code, params int
}

func (op Op) String() string {
	return fmt.Sprintf("Op{code: %d, params: %d}", op.code, op.params)
}

var OpAdd Op = Op{code: 01, params: 3}
var OpMlt Op = Op{code: 02, params: 3}
var OpInp Op = Op{code: 03, params: 1}
var OpOut Op = Op{code: 04, params: 1}
var OpHlt Op = Op{code: 99, params: 0}

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

func getParams(prog []int, offset int, modes []int) []int {
	params := make([]int, len(modes))
	for i := 0; i < len(params); i++ {
		mode := modes[i]
		switch mode {
		case 0:
			params[i] = prog[offset+i]
		case 1:
			params[i] = offset + i
		default:
			panic(fmt.Sprintf("Unexpected param mode: %d", mode))
		}
	}
	return params
}

func compute(prog []int) {
	var step int
	for i := 0; i < len(prog); i += step {
		op, modes := parseOp(prog[i])
		if op.code == OpHlt.code {
			break
		}
		params := getParams(prog, i+1, modes)
		switch op.code {
		case OpAdd.code:
			prog[params[2]] = prog[params[0]] + prog[params[1]]
		case OpMlt.code:
			prog[params[2]] = prog[params[0]] * prog[params[1]]
		case OpInp.code:
			reader := bufio.NewReader(os.Stdin)
			fmt.Print("Input: ")
			input, err := reader.ReadString('\n')
			check(err)
			n, err := strconv.Atoi(strings.Trim(input, "\n"))
			check(err)
			prog[params[0]] = n
		case OpOut.code:
			println(prog[params[0]])
		default:
			panic(fmt.Sprintf("Unexpected OpCode: %d", op.code))
		}
		step = op.params + 1
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
		num, err := strconv.Atoi(strings.Trim(s, "\n "))
		check(err)
		prog[i] = num
	}
	compute(prog)
}
