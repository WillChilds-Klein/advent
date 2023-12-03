import functools
import re
import sys


def main():
    lines = [l.strip() for l in sys.stdin.readlines()]
    schema = [[c for c in line] for line in lines]

    def get_gear(i: int, j: int) -> int:
        if schema[i][j] != '*':
            return 0
        ratios = []
        seen = [[False for _ in schema[0]] for _ in schema]
        def extract_num(x: int, y: int) -> int:
            start, end = y, y
            # look backwards
            for k in range(y, -1, -1):
                seen[x][k] = True
                if not schema[x][k] or not schema[x][k].isdigit():
                    break
                start = k
            # look forwards
            for k in range(y, len(schema[0])):
                seen[x][k] = True
                if not schema[x][k] or not schema[x][k].isdigit():
                    break
                end = k
            # if start == end, then the ratio is only 1 digit
            return int(''.join(schema[x][start:(end+1)]))

        for x in range(max(i-1, 0), min(i+1+1,len(schema))):
            for y in range(max(j-1, 0), min(j+1+1,len(schema[0]))):
                if seen[x][y] or not schema[x][y] or not schema[x][y].isdigit():
                    continue
                # we know here that schema[x][y] is an unseen digit
                ratios.append(extract_num(x, y))
        if len(ratios) != 2:
            return 0
        return functools.reduce(lambda x, y: x * y, ratios)

    # first pass clears non-symbols/non-numbers
    for i in range(len(schema)):
        for j in range(len(schema[0])):
            if schema[i][j] == '.':
                schema[i][j] = None
    # second pass calculates running sum and updates schema
    rsum = 0
    for i in range(len(schema)):
        for j in range(len(schema[0])):
            rsum += get_gear(i, j)
    print(rsum)


if __name__ == '__main__':
    main()
