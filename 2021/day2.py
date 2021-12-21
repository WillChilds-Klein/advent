import sys
import re

def main1(lines):
    x, z = 0, 0
    p = re.compile("^([a-z]+) ([\d]+)")
    for line in lines:
        r = p.search(line)
        cmd, v = r.group(1), int(r.group(2))
        if cmd == 'forward':
            x += v
        if cmd == 'up':
            z -= v
        if cmd == 'down':
            z += v
    print(x * z)

def main2(lines):
    x, z, aim = 0, 0, 0
    p = re.compile("^([a-z]+) ([\d]+)")
    for line in lines:
        r = p.search(line)
        cmd, v = r.group(1), int(r.group(2))
        if cmd == 'forward':
            x += v
            z += aim * v
        if cmd == 'up':
            aim -= v
        if cmd == 'down':
            aim += v
    print(x * z)

if __name__ == '__main__':
    lines = [l.strip() for l in sys.stdin.readlines()]
    main1(lines)
    main2(lines)
