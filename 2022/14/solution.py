import dataclasses
import sys
import typing


@dataclasses.dataclass
class Point:
    x: int
    y: int

    def __hash__(self) -> int:
        return hash((self.x, self.y))


ORIGIN = Point(500, 0)


def main():
    lines = list(map(str.strip, open(sys.argv[1]).readlines()))
    rocks = []
    for line in lines:
        p_ints = [[int(p) for p in c.split(",")] for c in line.split(" -> ")]
        rocks.append([Point(*p) for p in p_ints])

    max_x, max_y = -1, -1
    min_x, min_y = 2**32, 2**32
    for rock in rocks:
        for p in rock:
            if p.x > max_x:
                max_x = p.x
            if p.y > max_y:
                max_y = p.y
            if p.x < min_x:
                min_x = p.x
            if p.y > min_y:
                min_y = p.y

    smap = [[None for _ in range(max_y + 1)] for _ in range(max_x + 1)]
    for rock in rocks:
        print(rock)
        prev = rock.pop(0)
        while rock:
            print(rock)
            curr = rock.pop(0)
            x_diff, y_diff = curr.x - prev.x, curr.y - prev.y
            print(x_diff, y_diff)
            if prev.x != curr.x:
                step = x_diff // abs(x_diff)
                for i in range(prev.x, curr.x + step, step):
                    print(i)
                    smap[i][curr.y] = "#"
            if prev.y != curr.y:
                step = y_diff // abs(y_diff)
                for i in range(prev.y, curr.y + step, step):
                    smap[curr.x][i] = "#"
            prev = curr

    def drop_sand(smap: list[list[typing.Optional[str]]]) -> bool:
        x, y = ORIGIN.x, ORIGIN.y
        while y < max_y:
            if smap[x][y + 1] is None:
                y += 1
            elif smap[x - 1][y + 1] is None:
                x -= 1
                y += 1
            elif smap[x + 1][y + 1] is None:
                x += 1
                y += 1
            else:
                smap[x][y] = "o"
                return True
        return False

    count = 0
    while drop_sand(smap):
        count += 1
    # draw_map(smap)
    print("PART 1", count)


def draw_map(smap: list[list[typing.Optional[str]]]):
    min_x = 2**32
    for y in range(len(smap[0])):
        for x in range(len(smap)):
            if smap[x][y] and x < min_x:
                min_x = x
    for y in range(len(smap[0])):
        for x in range(len(smap)):
            if x < min_x:
                continue
            print(smap[x][y] if smap[x][y] else ".", end="")
        print()


if __name__ == "__main__":
    main()
