// Package etl implements transformation from our
// legacy system to the shiny new one.
package etl

import "strings"

// Transform takes a map of scores to array of tiles and returns a map of tiles to score.
// tile is downcased
// {1: {"A", "E", "I"}} -> {"a": 1, "e": 1, "i": 1}
func Transform(input map[int][]string) map[string]int {
    result := make(map[string]int)
    for score, letters := range input {
        for _, letter := range letters {
            result[strings.ToLower(letter)] = score
        }
    }
    return result
}
