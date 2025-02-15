package parsinglogfiles

import (
	"regexp"
	"strings"
)

func IsValidLine(text string) bool {
	return regexp.MustCompile(`^\[INF\]|^\[DBG\]|^\[WRN\]|^\[INF\]|^\[ERR\]|^\[FTL\]`).MatchString(text)
}

func SplitLogLine(text string) []string {
	return regexp.MustCompile(`<(~|\*|=|-)*>`).Split(text, -1)
}

func CountQuotedPasswords(lines []string) int {
	count := 0
	regularExpr := regexp.MustCompile(`\"(\s|\w)*(?i)(password)(\s|\w)*\"`)

	for _, line := range lines {
		if regularExpr.MatchString(line) {
			count += 1
		}

	}

	return count
}

func RemoveEndOfLineText(text string) string {
	return regexp.MustCompile(`end-of-line[0-9]*`).ReplaceAllString(text, "")
}

func TagWithUserName(lines []string) []string {
	updatedLogs := []string{}

	regExpr := regexp.MustCompile(`User\b(\s)*\w*\s`)

	for _, log := range lines {
		userStr := regExpr.FindString(log)
		if userStr != "" {
			data := regexp.MustCompile(`User(\s)*`).Split(userStr, -1)
			updatedLogs = append(updatedLogs, "[USR] "+strings.Trim(data[1], "")+log)

		} else {
			updatedLogs = append(updatedLogs, log)
		}
	}

	return updatedLogs
}
