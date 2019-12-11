package main

import (
	//"bufio"
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strconv"
	"strings"
)

// https://adventofcode.com/2019/day/7

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
	case OpIft.code:
		op = OpIft
	case OpIff.code:
		op = OpIff
	case OpLss.code:
		op = OpLss
	case OpEql.code:
		op = OpEql
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

func compute(prog []int, input []int) []int {
	output := make([]int, 0)
	ptr := 0
	for {
		op, modes := parseOp(prog[ptr])
		if op.code == OpHlt.code {
			break
		}
		params := getParams(prog, ptr+1, modes)
		switch op.code {
		case OpAdd.code:
			prog[params[2]] = prog[params[0]] + prog[params[1]]
		case OpMlt.code:
			prog[params[2]] = prog[params[0]] * prog[params[1]]
		case OpInp.code:
			if len(input) == 0 {
				panic("OpInp called with empty input queue!")
			}
			prog[params[0]] = input[0]
			input = input[1:]
		case OpOut.code:
			output = append(output, prog[params[0]])
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
		default:
			panic(fmt.Sprintf("Unexpected OpCode: %d", op.code))
		}
		ptr += op.params + 1
	}
	return output
}

func factorial(n int) int {
	if n == 1 {
		return 1
	}
	return n * factorial(n-1)
}

func swap(arr []int, i int, j int) {
	tmp := arr[i]
	arr[i] = arr[j]
	arr[j] = tmp
}

func reverse(arr []int, start int, end int) {
	for i := 0; i <= (end-start)/2; i++ {
		swap(arr, start+i, end-i)
	}
}

// algorithm in this method shamelessly ripped off from wikipedia:
// https://en.wikipedia.org/wiki/Permutation#Algorithms_to_generate_permutations
func permute(arr []int) bool {
	k, l := -1, -1
	for j := 0; j < len(arr)-1; j++ {
		if arr[j] < arr[j+1] {
			k = j
		}
	}
	if k < 0 {
		return false
	}
	for j := k + 1; j < len(arr); j++ {
		if arr[k] < arr[j] {
			l = j
		}
	}
	swap(arr, k, l)
	reverse(arr, k+1, len(arr)-1)
	return true
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

	phaseSettings := []int{0, 1, 2, 3, 4}
	sort.Ints(phaseSettings) // NOTE: need to ensure sort for perutations
	var output []int
	max := -1
	for hasNext := true; hasNext; {
		input := 0
		for _, phase := range phaseSettings {
			output = compute(prog, []int{phase, input})
			input = output[0]
		}
		if output[0] > max {
			max = output[0]
		}
		hasNext = permute(phaseSettings)
	}
	println(max)
}
