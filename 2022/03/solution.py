import math
import sys

def main():
    lines = open(sys.argv[1]).readlines()
    priority_sum = 0
    for line in lines:
        mid = math.floor(len(line)/2)
        left, right = set(line[:mid]), set(line[mid:])
        for x in set.intersection(left, right):
            priority_sum += priority(x)
    print("PART 1: " + str(priority_sum))
    badge_sum = 0
    for ii in range(0, len(lines), 3):
        one, two, three = lines[ii], lines[ii+1], lines[ii+2]
        for x in set.intersection(*map(set, map(str.strip, [one, two, three]))):
            badge_sum += priority(x)
    print("PART 2: " + str(badge_sum))

def priority(x: str) -> int:
    assert x and len(x) == 1 and x.isalpha()
    if x.upper() == x:
        return ord(x) - ord('A') + 27
    else:
        return ord(x) - ord('a') + 1


if __name__ == '__main__':
    main()
