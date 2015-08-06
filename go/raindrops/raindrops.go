package raindrops

import "strconv"

/*
Convert(int) converts a number to a string, the contents of which depends on the number's prime factors.

  - If the number contains 3 as a prime factor, output 'Pling'.
  - If the number contains 5 as a prime factor, output 'Plang'.
  - If the number contains 7 as a prime factor, output 'Plong'.
  - If the number does not contain 3, 5, or 7 as a prime factor,
    just pass the number's digits straight through.
*/
func Convert(num int) (result string) {
    if num%3 == 0 {
        result += "Pling"
    }
    if num%5 == 0 {
        result += "Plang"
    }
    if num%7 == 0 {
        result += "Plong"
    }
    if result == "" {
        result = strconv.Itoa(num)
    }
    return
}
