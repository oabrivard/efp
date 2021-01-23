package main

import (
	"fmt"
	"math"
	"strconv"
)

func readPositiveFloat() float64 {
	for {
		var line string
		_, err := fmt.Scanln(&line)
		if err != nil {
			panic(err)
		}

		if value, err := strconv.ParseFloat(line, 64); err == nil && value >= 0 {
			return value
		}

		fmt.Print("Please enter a value >= 0 : ")
	}
}

func main() {
	fmt.Print("What is the bill? ")
	bill := readPositiveFloat()

	fmt.Print("What is the tip percentage? ")
	tipPercent := readPositiveFloat()

	tip := math.Round(tipPercent*bill) / 100

	fmt.Printf("The tip is %.2f\nThe total is %.2f\n", tip, bill+tip)
}
