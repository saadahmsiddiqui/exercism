package airportrobot

import "fmt"

// Write your code here.
// This exercise does not have tests for each individual task.
// Try to solve all the tasks first before running the tests.

type Greeter interface {
	Language() string
	Greet(name string) string
}

type Italian struct {
}

func (i Italian) Greet(name string) string {
	return fmt.Sprintf("I can speak Italian: Ciao %s!", name)
}

func (i Italian) Language() string {
	return "Italian"
}

type Portuguese struct {
}

func (p Portuguese) Greet(name string) string {
	return fmt.Sprintf("I can speak Portuguese: Olá %s!", name)
}

func (p Portuguese) Language() string {
	return "Portuguese"
}

func SayHello(name string, greeter Greeter) string {
	return greeter.Greet(name)
}
