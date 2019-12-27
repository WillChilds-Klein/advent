package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strings"
)

// https://adventofcode.com/2019/day/14

func check(err error) {
	if err != nil {
		fmt.Println(err)
		panic(err)
	}
}

func ore(recipes map[string]map[string]int, outputs map[string]int, curr string) int {
	if curr == "ORE" {
		return 1
	}
	sum := 0
	for component, count := range recipes[curr] {
		blockSize, present := outputs[component]
		if !present {
			blockSize = 1
		}
		ceilCount := int(math.Ceil(float64(count) / float64(blockSize)))
		fmt.Println("ceilCount", component, count, ceilCount, float64(count), float64(outputs[component]))
		sum += ceilCount * ore(recipes, outputs, component)
	}
	fmt.Println(curr, sum)
	return sum
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)
	ss := strings.Split(strings.Trim(string(file), "\n"), "\n")
	recipes := make(map[string]map[string]int, 0)
	outputs := make(map[string]int, 0)
	for _, s := range ss {
		factors := strings.Split(s, " => ")
		if len(factors) != 2 {
			panic("Too many '=>' in Chemical str")
		}
		var count int
		var name string
		_, err := fmt.Sscanf(factors[1], "%d %s", &count, &name)
		check(err)
		outputs[name] = count
		components := make(map[string]int, 0)
		componentStrs := strings.Split(factors[0], ", ")
		for _, componentStr := range componentStrs {
			var componentCount int
			var componentName string
			_, err := fmt.Sscanf(componentStr, "%d %s", &componentCount, &componentName)
			check(err)
			components[componentName] = componentCount
		}
		recipes[name] = components
	}
	fmt.Println("recipes", recipes)
	fmt.Println("outputs", outputs)
	fmt.Println("ore", 1*ore(recipes, outputs, "FUEL"))
}
