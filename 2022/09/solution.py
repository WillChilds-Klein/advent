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
    rope = [Point(0, 0) for _ in range(2)]
    print("PART 1:", len(execute(moves, rope)))
    rope = [Point(0, 0) for _ in range(10)]
    print("PART 2:", len(execute(moves, rope)))


def execute(moves: list[tuple[str, int]], rope: list[Point]) -> set[Point]:
    visited = set([rope[-1]])
    for d, m in moves:
        for _ in range(m):
            move_rope(d, rope)
            visited.add(copy.deepcopy(rope[-1]))
            # draw_rope(rope)
    return visited


def move_rope(d: str, rope: list[Point]):
    h = rope[0]
    if d == "R":
        h.x += 1
    if d == "L":
        h.x -= 1
    if d == "U":
        h.y += 1
    if d == "D":
        h.y -= 1
    prev = h
    for knot in rope[1:]:
        if abs(prev.x - knot.x) <= 1 and abs(prev.y - knot.y) <= 1:
            prev = knot
            continue
        x_dir = 1 if prev.x - knot.x >= 1 else -1 if prev.x - knot.x <= -1 else 0
        y_dir = 1 if prev.y - knot.y >= 1 else -1 if prev.y - knot.y <= -1 else 0
        knot.x += x_dir
        knot.y += y_dir
        prev = knot


def draw_rope(rope: list[Point], board_size=None):
    min_x, min_y = 2**31, 2**31
    max_x, max_y = -1 * 2**31, -1 * 2**31
    for knot in rope:
        if knot.x < min_x:
            min_x = knot.x
        if knot.x > max_x:
            max_x = knot.x
        if knot.y < min_y:
            min_y = knot.y
        if knot.y > max_y:
            max_y = knot.y
    if board_size:
        board = [["." for _ in range(board_size)] for _ in range(board_size)]
    else:
        board = [
            ["." for _ in range(abs(min_y) + abs(max_y) + 1)]
            for _ in range(abs(min_x) + abs(max_x) + 1)
        ]
    board[abs(min_x)][abs(min_y)] = "s"
    for ii in range(len(rope)):
        x, y = abs(min_x) + rope[ii].x, abs(min_y) + rope[ii].y
        board[x][y] = str(ii)
    for row in board:
        print("".join(row))
    print()


if __name__ == "__main__":
    main()
