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
	orbits := make(map[string]string)

	for _, s := range ss {
		planets := strings.Split(s, ")")
		orbitee, orbiter := planets[0], planets[1]
		orbits[orbiter] = orbitee
	}

	sum := 0
	for orbiter, _ := range orbits {
		ctr := 0
		for planet := orbiter; planet != "COM"; planet = orbits[planet] {
			ctr++
		}
		sum += ctr
	}
	println(sum)
}
