package chessboard

// Declare a type named File which stores if a square is occupied by a piece - this will be a slice of bools
type File = []bool

// Declare a type named Chessboard which contains a map of eight Files, accessed with keys from "A" to "H"
type Chessboard = map[string]File

// CountInFile returns how many squares are occupied in the chessboard,
// within the given file.
func CountInFile(cb Chessboard, file string) int {
	count := 0
	for _, occupied := range cb[file] {
		if occupied {
			count += 1
		}
	}
	return count
}

// CountInRank returns how many squares are occupied in the chessboard,
// within the given rank.
func CountInRank(cb Chessboard, rank int) int {
	count := 0
	idxRank := rank - 1
	if idxRank < 0 || idxRank >= 8 {
		return count
	}

	for _, val := range cb {
		if val[idxRank] {
			count += 1
		}
	}

	return count
}

// CountAll should count how many squares are present in the chessboard.
func CountAll(cb Chessboard) int {
	count := 0

	for _, file := range cb {
		count += len(file)
	}

	return count
}

// CountOccupied returns how many squares are occupied in the chessboard.
func CountOccupied(cb Chessboard) int {
	occupied := 0

	for k, _ := range cb {
		occupied += CountInFile(cb, k)
	}

	return occupied
}
