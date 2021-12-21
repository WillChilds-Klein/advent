import sys
import re

def main1(lines):
    count = 0
    prev = 2**31-1
    for line in lines:
        val = int(line.strip())
        if val > prev:
            count += 1
        prev = val
    print(count)

def main2(lines):
    count = 0
    prev = 2**31-1
    window = [None for _ in range(3)]
    for line in lines:
        val = int(line.strip())
        window.pop(0)
        window.append(val)
        if all(window):
            if sum(window) > prev:
                count += 1
            prev = sum(window)
    print(count)

if __name__ == '__main__':
    lines = [l.strip() for l in sys.stdin.readlines()]
    main1(lines)
    main2(lines)
