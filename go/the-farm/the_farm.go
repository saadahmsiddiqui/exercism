package thefarm

import (
	"errors"
	"fmt"
)

// TODO: define the 'DivideFood' function
func DivideFood(calc FodderCalculator, numCows int) (float64, error) {
	fodderAmnt, err := calc.FodderAmount(numCows)

	if err != nil {
		return 0, err
	}

	fatteningFactor, err := calc.FatteningFactor()

	if err != nil {
		return 0, err
	}

	foodPerCow := fodderAmnt * fatteningFactor / float64(numCows)
	return foodPerCow, nil
}

// TODO: define the 'ValidateInputAndDivideFood' function
func ValidateInputAndDivideFood(calc FodderCalculator, numCows int) (float64, error) {
	if numCows > 0 {
		v, err := DivideFood(calc, numCows)
		return v, err
	} else {
		return 0, errors.New("invalid number of cows")
	}
}

type InvalidCowsError struct {
	numberOfCows int
	message      string
}

func (e *InvalidCowsError) Error() string {
	return fmt.Sprintf("%d cows are invalid: %s", e.numberOfCows, e.message)
}

// TODO: define the 'ValidateNumberOfCows' function
func ValidateNumberOfCows(numCows int) error {
	if numCows < 0 {
		return &InvalidCowsError{numberOfCows: numCows, message: "there are no negative cows"}
	}

	if numCows == 0 {
		return &InvalidCowsError{numberOfCows: numCows, message: "no cows don't need food"}
	}

	return nil
}

// Your first steps could be to read through the tasks, and create
// these functions with their correct parameter lists and return types.
// The function body only needs to contain `panic("")`.
//
// This will make the tests compile, but they will fail.
// You can then implement the function logic one by one and see
// an increasing number of tests passing as you implement more
// functionality.
