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

func compute(prog []int, input chan int, output chan int) {
	ptr := 0
	for {
		op, modes := parseOp(prog[ptr])
		if op.code == OpHlt.code {
			close(output)
			return
		}
		params := getParams(prog, ptr+1, modes)
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
		default:
			panic(fmt.Sprintf("Unexpected OpCode: %d", op.code))
		}
		ptr += op.params + 1
	}
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

	phases := []int{0, 1, 2, 3, 4}
	sort.Ints(phases) // NOTE: need to ensure sort for permutation
	max := -1
	for hasNext := true; hasNext; {
		channels := make([]chan int, len(phases)+1)
		channels[0] = make(chan int)
		for i, phase := range phases {
			channels[i+1] = make(chan int)
			progCopy := make([]int, len(prog))
			copy(progCopy, prog)
			go compute(progCopy, channels[i], channels[i+1])
			channels[i] <- phase
		}
		channels[0] <- 0
		close(channels[0])
		result := <-channels[len(phases)]
		if result > max {
			max = result
		}
		hasNext = permute(phases)
	}
	println(max)
}
