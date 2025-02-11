package chance

import "math/rand"

// RollADie returns a random int d with 1 <= d <= 20.
func RollADie() int {
	return rand.Intn(19) + 1
}

// GenerateWandEnergy returns a random float64 f with 0.0 <= f < 12.0.
func GenerateWandEnergy() float64 {
	return rand.Float64() * 12
}

// ShuffleAnimals returns a slice with all eight animal strings in random order.
func ShuffleAnimals() []string {
	animals := []string{"ant", "beaver", "cat", "dog", "elephant", "fox", "giraffe", "hedgehog"}
	added := map[string]bool{}
	shuffled := []string{}

	for range animals {
		randIdx := rand.Intn(len(animals))

		for added[animals[randIdx]] {
			randIdx = rand.Intn(len(animals))
		}

		shuffled = append(shuffled, animals[randIdx])
		added[animals[randIdx]] = true
	}

	return shuffled
}
