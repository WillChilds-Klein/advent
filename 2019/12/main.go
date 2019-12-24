package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strings"
)

// https://adventofcode.com/2019/day/12

func check(err error) {
	if err != nil {
		fmt.Println(err)
		panic(err)
	}
}

type Point struct {
	x, y, z int
}

type Body struct {
	pos, vel Point
}

func (body Body) String() string {
	pos, vel := body.pos, body.vel
	return fmt.Sprintf("pos={x:\t%d,\ty:\t%d,\tz:\t%d}\tvel={x:\t%d,\ty:\t%d,\tz:\t%d}", pos.x, pos.y, pos.z, vel.x, vel.y, vel.z)
}

func (body *Body) Parse(str string) error {
	_, err := fmt.Sscanf(str, "<x=%d, y=%d, z=%d>", &body.pos.x, &body.pos.y, &body.pos.z)
	return err
}

func printBodies(bodies []Body) {
	for _, body := range bodies {
		fmt.Println(body)
	}
}

func updateVelocity(body, other int) int {
	if body > other {
		return -1
	} else if body < other {
		return 1
	}
	return 0
}

func updateVelocities(bodies []Body) []Body {
	newBodies := make([]Body, len(bodies))
	for i, body := range bodies {
		newBodies[i] = Body{pos: body.pos, vel: body.vel}
		for j, other := range bodies {
			if i == j {
				continue
			}
			newBodies[i].vel.x += updateVelocity(body.pos.x, other.pos.x)
			newBodies[i].vel.y += updateVelocity(body.pos.y, other.pos.y)
			newBodies[i].vel.z += updateVelocity(body.pos.z, other.pos.z)
		}
	}
	return newBodies
}

func updatePositions(bodies []Body) []Body {
	newBodies := make([]Body, len(bodies))
	for i, body := range bodies {
		newBodies[i] = Body{pos: body.pos, vel: body.vel}
		newBodies[i].pos.x += body.vel.x
		newBodies[i].pos.y += body.vel.y
		newBodies[i].pos.z += body.vel.z
	}
	return newBodies
}

func abs(x int) int {
	return int(math.Abs(float64(x)))
}

func calculateTotalEnergy(body Body) int {
	potential := abs(body.pos.x) + abs(body.pos.y) + abs(body.pos.z)
	kinetic := abs(body.vel.x) + abs(body.vel.y) + abs(body.vel.z)
	return potential * kinetic
}

func main() {
	if len(os.Args) != 2 {
		panic("No input file specified!")
	}
	fname := os.Args[1]
	file, err := ioutil.ReadFile(fname)
	check(err)
	ss := strings.Split(strings.Trim(string(file), "\n"), "\n")
	bodies := make([]Body, len(ss))
	for i, s := range ss {
		var body Body
		err := body.Parse(s)
		check(err)
		bodies[i] = body
	}

	for i := 0; i < 1000; i++ {
		bodies = updateVelocities(bodies)
		bodies = updatePositions(bodies)
	}

	sum := 0
	for _, body := range bodies {
		sum += calculateTotalEnergy(body)
	}
	fmt.Println(sum)
}
