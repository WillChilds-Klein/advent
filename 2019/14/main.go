package main

import (
	"errors"
	"fmt"
	"io/ioutil"
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

type Chemical struct {
	name       string
	count      int
	components map[string]int
}

func (chem Chemical) String() string {
	components := make([]string, len(chem.components))
	i := 0
	for component, count := range chem.components {
		components[i] = fmt.Sprintf("%d %s", count, component)
		i++
	}
	result := fmt.Sprintf("%d %s", chem.count, chem.name)
	return strings.Join([]string{strings.Join(components, ", "), result}, " => ")
}

func (chem *Chemical) Parse(str string) error {
	factors := strings.Split(str, " => ")
	if len(factors) != 2 {
		return errors.New("Too many '=>' in Chemical str")
	}
	_, err := fmt.Sscanf(factors[1], "%d %s", &chem.count, &chem.name)
	if err != nil {
		return err
	}
	var count int
	var name string
	components := strings.Split(factors[0], ", ")
	for _, component := range components {
		_, err := fmt.Sscanf(component, "%d %s", &count, &name)
		if err != nil {
			return err
		}
		chem.components[name] = count
	}
	return err
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)
	ss := strings.Split(strings.Trim(string(file), "\n"), "\n")
	for _, s := range ss {
		var chem Chemical
		chem.components = make(map[string]int, 0)
		err := chem.Parse(s)
		check(err)
		fmt.Println(chem)
	}
}
