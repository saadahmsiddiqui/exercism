package expenses

import (
	"errors"
	"fmt"
)

// Record represents an expense record.
type Record struct {
	Day      int
	Amount   float64
	Category string
}

// DaysPeriod represents a period of days for expenses.
type DaysPeriod struct {
	From int
	To   int
}

// Filter returns the records for which the predicate function returns true.
func Filter(in []Record, predicate func(Record) bool) []Record {
	filtered := []Record{}

	for _, r := range in {
		if predicate(r) {
			filtered = append(filtered, r)
		}
	}

	return filtered
}

// ByDaysPeriod returns predicate function that returns true when
// the day of the record is inside the period of day and false otherwise.
func ByDaysPeriod(p DaysPeriod) func(Record) bool {
	return func(r Record) bool {
		return r.Day >= p.From && r.Day <= p.To
	}
}

// ByCategory returns predicate function that returns true when
// the category of the record is the same as the provided category
// and false otherwise.
func ByCategory(c string) func(Record) bool {
	return func(r Record) bool {
		return r.Category == c
	}
}

// TotalByPeriod returns total amount of expenses for records
// inside the period p.
func TotalByPeriod(in []Record, p DaysPeriod) float64 {
	totalAmount := 0.0

	for _, r := range Filter(in, ByDaysPeriod(p)) {
		totalAmount += r.Amount
	}

	return totalAmount
}

// CategoryExpenses returns total amount of expenses for records
// in category c that are also inside the period p.
// An error must be returned only if there are no records in the list that belong
// to the given category, regardless of period of time.
func CategoryExpenses(in []Record, p DaysPeriod, c string) (float64, error) {
	cats := map[string]bool{}

	for _, r := range in {
		cats[r.Category] = true
	}

	_, ok := cats[c]

	if !ok {
		errorStr := fmt.Sprintf("unknown category %s", c)
		return 0, errors.New(errorStr)
	}

	inDaysPeriod := Filter(in, ByDaysPeriod(p))
	inCategory := Filter(inDaysPeriod, ByCategory(c))
	totalExpense := 0.0

	for _, r := range inCategory {
		totalExpense += r.Amount
	}

	return totalExpense, nil

}
