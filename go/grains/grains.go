package grains

import "errors"
import "math"

// power_of_two(n) returns 2 raised to the nth power as an uint.
func power_of_two(n int) uint64 {
    return uint64(math.Pow(2.0, float64(n)))
}

// Square(n) calculates the number of grains on each square.
// 1 on the first, 2 on the second, ... 2^(n-1) on the nth
func Square(n int) (uint64, error) {
    if n > 64 || n < 1 {
        return 0, errors.New("invalid chess board square")
    }
    return power_of_two(n - 1), nil
}

// Total() returns the sum of the first 64 squares
//   using: 1 + 2 + ... 2^(n-1) = -1 + 2^n
func Total() uint64 {
    return power_of_two(65) - 1
}
