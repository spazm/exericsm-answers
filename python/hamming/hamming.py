def distance(s1, s2):
    if len(s1) != len(s2):
        raise UnequalStringLength

    diff = 0
    for c1, c2 in zip(s1, s2):
        if c1 != c2:
            diff += 1
    return diff

class UnequalStringLength(Exception):
    pass
