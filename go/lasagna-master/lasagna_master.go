package lasagna

// TODO: define the 'PreparationTime()' function
func PreparationTime(layers []string, timePerLayer int) int {
	time := timePerLayer

	if timePerLayer <= 0 {
		time = 2
	}

	return len(layers) * time
}

// TODO: define the 'Quantities()' function
func Quantities(layers []string) (noodlesWeight int, sauceWeight float64) {
	for i := 0; i < len(layers); i++ {
		if layers[i] == "noodles" {
			noodlesWeight += 50
		} else if layers[i] == "sauce" {
			sauceWeight += 0.2
		}
	}

	return noodlesWeight, sauceWeight
}

// TODO: define the 'AddSecretIngredient()' function
func AddSecretIngredient(friendsList, myList []string) {
	myList[len(myList)-1] = friendsList[len(friendsList)-1]
}

// TODO: define the 'ScaleRecipe()' function
func ScaleRecipe(amounts []float64, portionsToCook int) []float64 {
	var newAmounts []float64

	for i := 0; i < len(amounts); i++ {
		multiplier := amounts[i] / 2
		weight := multiplier * float64(portionsToCook)
		newAmounts = append(newAmounts, weight)
	}

	return newAmounts
}

// Your first steps could be to read through the tasks, and create
// these functions with their correct parameter lists and return types.
// The function body only needs to contain `panic("")`.
//
// This will make the tests compile, but they will fail.
// You can then implement the function logic one by one and see
// an increasing number of tests passing as you implement more
// functionality.
