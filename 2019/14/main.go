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
	name  string
	count int
}

func (chem *Chemical) Parse(str string) error {
	_, err := fmt.Sscanf(str, "%d %s", &chem.count, &chem.name)
	return err
}

func (chem Chemical) String() string {
	return fmt.Sprintf("%d %s", chem.count, chem.name)
}

func printRecipe(chem Chemical, components []Chemical) string {
	componentStrs := make([]string, len(components))
	for i, component := range components {
		componentStrs[i] = component.String()
	}
	return strings.Join([]string{strings.Join(componentStrs, ", "), chem.String()}, " => ")
}

func parseRecipe(str string) (Chemical, []Chemical, error) {
	var chem Chemical
	factors := strings.Split(str, " => ")
	componentStrs := strings.Split(factors[0], ", ")
	components := make([]Chemical, len(componentStrs))
	if len(factors) != 2 {
		return chem, components, errors.New("Too many '=>' in Chemical str")
	}
	err := chem.Parse(factors[1])
	if err != nil {
		return chem, components, err
	}
	for i, componentStr := range componentStrs {
		var component Chemical
		err := component.Parse(componentStr)
		if err != nil {
			return chem, components, err
		}
		components[i] = component
	}
	return chem, components, nil
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)
	ss := strings.Split(strings.Trim(string(file), "\n"), "\n")
	recipes := make(map[Chemical][]Chemical, 0)
	for _, s := range ss {
		chem, components, err := parseRecipe(s)
		check(err)
		recipes[chem] = components
	}
	fmt.Println(recipes)
}
