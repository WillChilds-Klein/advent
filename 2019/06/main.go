package main

import (
	"io/ioutil"
	"os"
	"strings"
)

// https://adventofcode.com/2019/day/6

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)

	ss := strings.Fields(string(file))
	orbits := make(map[string][]string)

	for _, s := range ss {
		planets := strings.Split(s, ")")
		orbitee, orbiter := planets[0], planets[1]
		orbits[orbiter] = append(orbits[orbiter], orbitee)
		orbits[orbitee] = append(orbits[orbitee], orbiter)
	}

	// init depth to -1 because we're starting one away from planet YOU orbits
	println(dfs("YOU", "", -1, orbits))
}

// NOTE: this should work in the general case of multiple orbiters
func dfs(curr string, prev string, depth int, orbits map[string][]string) int {
	sum := 0
	for _, neighbor := range orbits[curr] {
		if neighbor == prev {
			continue
		}
		if neighbor == "SAN" {
			return depth
		}
		sum += dfs(neighbor, curr, depth+1, orbits)
	}
	return sum
}
