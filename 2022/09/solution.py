import copy
import dataclasses
import sys


@dataclasses.dataclass
class Point:
    x: int
    y: int

    def __hash__(self) -> int:
        return hash((self.x, self.y))


def main():
    lines = open(sys.argv[1]).readlines()
    moves = [line.strip().split(" ") for line in lines]
    moves = list(map(lambda x: (x[0], int(x[1])), moves))
    s, t, h = Point(0, 0), Point(0, 0), Point(0, 0)
    visited = set([t])
    for d, m in moves:
        for _ in range(m):
            move_rope(d, h, t)
            visited.add(copy.deepcopy(t))
    print("PART 1:", len(visited))


def move_rope(d: str, h: Point, t: Point):
    if d == "R":
        h.x += 1
    if d == "L":
        h.x -= 1
    if d == "U":
        h.y += 1
    if d == "D":
        h.y -= 1

    if abs(h.x - t.x) <= 1 and abs(h.y - t.y) <= 1:
        return

    if d == "R":
        t.x += 1
        t.y = h.y
    if d == "L":
        t.x -= 1
        t.y = h.y
    if d == "U":
        t.x = h.x
        t.y += 1
    if d == "D":
        t.x = h.x
        t.y -= 1


if __name__ == "__main__":
    main()
