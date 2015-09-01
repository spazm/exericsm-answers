def is_leap_year(year):
    """
    A year is a leap year if it is divisible by 4,
    excepting years divisible by 100 unless they are
    also divisible by 400.
    """

    def has_factor(f):
        return year % f == 0

    return (has_factor(4) and
            (not has_factor(100) or has_factor(400)))
