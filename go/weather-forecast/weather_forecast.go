// Package weather provides weather information of an area.
package weather

// CurrentCondition represents current weather conditions.
var CurrentCondition string

// CurrentLocation represent the location tracked by the package.
var CurrentLocation string

// Forecast function accepts a city an condition as argument and returns a forecast formatted string.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
