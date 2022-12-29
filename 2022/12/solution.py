import sys

INT_MAX = 2 ** 32 - 1

Point = tuple[int,int]

def main():
    sys.setrecursionlimit(10000)
    lines = open(sys.argv[1]).readlines()
    hmap = [[c for c in line.strip() ] for line in lines]

    s, e = None, None
    for i in range(len(hmap)):
        for j in range(len(hmap[0])):
            if hmap[i][j] == 'S':
                s = (i, j)
                hmap[i][j] = 'a'
            elif hmap[i][j] == 'E':
                e = (i, j)
                hmap[i][j] = 'z'
            hmap[i][j] = ord(hmap[i][j]) - ord('a')

    def is_in_bounds(p: Point) -> bool:
        return p[0] >= 0 and p[0] < len(hmap) and p[1] >= 0 and p[1] < len(hmap[0])

    def is_valid_height(p: Point, candidate: Point) -> bool:
        return hmap[candidate[0]][candidate[1]] - hmap[p[0]][p[1]] <= 1

    def explore(s: Point, e: Point, seen: set[Point]) -> int:
        seen.add(s)
        if s == e:
            return 0
        x, y = s
        candidates = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
        candidates = [c for c in candidates if c not in seen]
        candidates = [c for c in candidates if is_in_bounds(c)]
        candidates = [c for c in candidates if is_valid_height(s, c)]
        candidates = [explore(c, e, set(seen)) for c in candidates]
        if not candidates:
            return INT_MAX
        return 1 + (candidates[0] if len(candidates) == 1 else min(*candidates))

    print("PART 1:", explore(s, e, set()))


if __name__ == '__main__':
    main()
