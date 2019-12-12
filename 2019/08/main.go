package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

// https://adventofcode.com/2019/day/8

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
	for i, row := range layer {
		for j := range row {
			switch layer[i][j] {
			case 0:
				fmt.Print(" ")
			case 1:
				fmt.Print("█")
			default:
				fmt.Print(layer[i][j])
			}
		}
		fmt.Print("\n")
	}
}

func decodeLayers(raw []int) [][][]int {
	numLayers := len(raw) / LayerHeight / LayerWidth
	layers := make([][][]int, numLayers)
	for layerIdx := 0; layerIdx < numLayers; layerIdx++ {
		layer := make([][]int, LayerHeight)
		for i := 0; i < LayerHeight; i++ {
			row := make([]int, LayerWidth)
			for j := 0; j < LayerWidth; j++ {
				rawIdx := (layerIdx * LayerHeight * LayerWidth) + (i * LayerWidth) + j
				row[j] = raw[rawIdx]
			}
			layer[i] = row
		}
		layers[layerIdx] = layer
	}
	return layers
}

func flattenLayers(layers [][][]int) [][]int {
	flattened := make([][]int, LayerHeight)
	for i := range flattened {
		flattened[i] = make([]int, LayerWidth)
		for j := range flattened[i] {
			flattened[i][j] = 2
		}
	}
	for _, layer := range layers {
		for i := 0; i < LayerHeight; i++ {
			for j := 0; j < LayerWidth; j++ {
				if flattened[i][j] == 2 {
					flattened[i][j] = layer[i][j]
				}
			}
		}
	}
	return flattened
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

	layers := decodeLayers(raw)
	image := flattenLayers(layers)
	printLayer(image)
}
