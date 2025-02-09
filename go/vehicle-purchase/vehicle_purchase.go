package purchase

import (
	"fmt"
	"strings"
)

// NeedsLicense determines whether a license is needed to drive a type of vehicle. Only "car" and "truck" require a license.
func NeedsLicense(kind string) bool {
	if kind == "car" || kind == "truck" {
		return true
	}

	return false
}

// ChooseVehicle recommends a vehicle for selection. It always recommends the vehicle that comes first in lexicographical order.
func ChooseVehicle(option1, option2 string) string {
	comparison := strings.Compare(option1, option2)
	var choice string
	if comparison > 0 {
		choice = option2
	} else {
		choice = option1
	}
	return fmt.Sprintf("%s is clearly the better choice.", choice)
}

// CalculateResellPrice calculates how much a vehicle can resell for at a certain age.
func CalculateResellPrice(originalPrice, age float64) float64 {
	if age < 10 {
		if age < 3 {
			return originalPrice * 0.8
		}

		return originalPrice * 0.7
	}

	return originalPrice * 0.5
}
