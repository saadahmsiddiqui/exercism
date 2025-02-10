package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	switch {
	case card == "ace":
		return 11
	case card == "two":
		return 2
	case card == "three":
		return 3
	case card == "four":
		return 4
	case card == "five":
		return 5
	case card == "six":
		return 6
	case card == "seven":
		return 7
	case card == "eight":
		return 8
	case card == "nine":
		return 9
	case card == "king" || card == "queen" || card == "jack" || card == "ten":
		return 10
	default:
		return 0
	}
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	card1Value := ParseCard(card1)
	card2Value := ParseCard(card2)

	sum := card1Value + card2Value
	dealerValue := ParseCard(dealerCard)

	switch {
	case sum <= 11:
		return "H"
	case sum == 22:
		return "P"
	case sum == 21:
		if dealerValue < 10 {
			return "W"
		}
		return "S"
	case sum >= 17 && sum <= 20:
		return "S"
	case sum >= 12 && sum <= 16:
		if dealerValue >= 7 {
			return "H"
		}
		return "S"
	}

	return ""
}
