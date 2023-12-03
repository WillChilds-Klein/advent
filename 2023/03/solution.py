import functools
import re
import sys


def main():
    lines = [l.strip() for l in sys.stdin.readlines()]
    schema = [[c for c in line] for line in lines]

    def part_num(i: int, j: int) -> int:
        if schema[i][j] is None or not schema[i][j].isdigit():
            return 0
        digits = []
        # determine number boundaries
        for x in range(j, len(schema[0])):
            if schema[i][x] is None or not schema[i][x].isdigit():
                break
            digits.append(schema[i][x])
        assert all(map(str.isdigit, digits))
        # after saving off digits, erase from |schema| so we don't overcount
        for x in range(j, j+len(digits)):
            assert schema[i][x].isdigit()
            schema[i][x] = None
        is_valid = False
        for x in range(max(i-1, 0), min(i+1+1,len(schema))):
            for y in range(max(j-1, 0), min(j+len(digits)+1,len(schema[0]))):
                # did we find a symbol?
                if schema[x][y] is not None and not schema[x][y].isdigit():
                    is_valid = True
                    break
        if not is_valid:
            return 0    # couldn't find any surrounding symbols
        return int(''.join(digits))

    # first pass clears non-symbols/non-numbers
    for i in range(len(schema)):
        for j in range(len(schema[0])):
            if schema[i][j] == '.':
                schema[i][j] = None
    # second pass calculates running sum and updates schema
    rsum = 0
    for i in range(len(schema)):
        for j in range(len(schema[0])):
            rsum += part_num(i, j)
    print(rsum)


if __name__ == '__main__':
    main()
