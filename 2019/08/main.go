package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strconv"
	"strings"
)

const (
	LayerWidth  = 25
	LayerHeight = 6
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func printLayer(layer [][]int) {
	for _, row := range layer {
		fmt.Println(row)
	}
	fmt.Println()
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)
	ss := strings.Split(string(file), "")
	raw := make([]int, len(ss))
	for i, s := range ss {
		num, err := strconv.Atoi(strings.Trim(s, "\n "))
		check(err)
		raw[i] = num
	}

	numLayers := len(raw) / LayerHeight / LayerWidth
	layers := make([][][]int, numLayers)
	minZeroes, minZeroesIdx := math.MaxInt64, -1
	for layerIdx := 0; layerIdx < numLayers; layerIdx++ {
		zeroes := 0
		layer := make([][]int, LayerHeight)
		for i := 0; i < LayerHeight; i++ {
			row := make([]int, LayerWidth)
			for j := 0; j < LayerWidth; j++ {
				rawIdx := (layerIdx * LayerHeight * LayerWidth) + (i * LayerWidth) + j
				row[j] = raw[rawIdx]
				if row[j] == 0 {
					zeroes++
				}
			}
			layer[i] = row
		}
		if zeroes < minZeroes {
			minZeroes = zeroes
			minZeroesIdx = layerIdx
		}
		layers[layerIdx] = layer
	}
	ones, twos := 0, 0
	for _, row := range layers[minZeroesIdx] {
		for _, num := range row {
			switch num {
			case 1:
				ones++
			case 2:
				twos++
			}
		}
	}
	fmt.Println(ones * twos)
}
