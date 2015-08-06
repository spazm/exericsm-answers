package leap

/* IsLeapYear(year) returns true if year is a leap-year.
   leap year is defined as a year that is divisible by 4
     except ever year that is divisible by 100
       unless the year is also divisible by 400
*/
func IsLeapYear(year int) bool {
    switch {
    case year%400 == 0:
        return true
    case year%100 == 0:
        return false
    case year%4 == 0:
        return true
    }
    return false
}
