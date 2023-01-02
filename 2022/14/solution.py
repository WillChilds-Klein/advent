import dataclasses
import json
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

    # compensate for floor. additional x space given by 45º right triangle between
    # origin, max fill point w/ floor, and point on floor directly below origin. length
    # of edge between the latter 2 is equal to the distance from origin to floor.
    max_y += 2
    max_x += max_y - (max_x - ORIGIN.x)

    smap_original = [[None for _ in range(max_y + 1)] for _ in range(max_x + 1)]
    for rock in rocks:
        prev = rock.pop(0)
        while rock:
            curr = rock.pop(0)
            x_diff, y_diff = curr.x - prev.x, curr.y - prev.y
            if prev.x != curr.x:
                step = x_diff // abs(x_diff)
                for i in range(prev.x, curr.x + step, step):
                    smap_original[i][curr.y] = "#"
            if prev.y != curr.y:
                step = y_diff // abs(y_diff)
                for i in range(prev.y, curr.y + step, step):
                    smap_original[curr.x][i] = "#"
            prev = curr

    def drop_sand(s: list[list[typing.Optional[str]]]) -> bool:
        y_lim = len(s[0]) - 1
        x, y = ORIGIN.x, ORIGIN.y
        while y < y_lim and s[x][y] is None:
            if s[x][y + 1] is None:
                y += 1
            elif s[x - 1][y + 1] is None:
                x -= 1
                y += 1
            elif s[x + 1][y + 1] is None:
                x += 1
                y += 1
            else:
                s[x][y] = "o"
                return True
        return False

    count = 0
    smap = json.loads(json.dumps(smap_original))
    while drop_sand(smap):
        count += 1
    # draw_map(smap)
    print("PART 1", count)

    count = 0
    smap = json.loads(json.dumps(smap_original))
    # add the floor
    floor_start, floor_end  = ORIGIN.x - max_y, ORIGIN.x + max_y
    for i in range(floor_start, floor_end+1):
        smap[i][-1] = "#"
    while drop_sand(smap):
        count += 1
    # draw_map(smap)
    print("PART 2", count)


def draw_map(s: list[list[typing.Optional[str]]]):
    min_x = 2**32
    for y in range(len(s[0])):
        for x in range(len(s)):
            if s[x][y] and x < min_x:
                min_x = x
    print(len(s), len(s[0]))
    for y in range(len(s[0])):
        for x in range(len(s)):
            if x < min_x:
                continue
            print(s[x][y] if s[x][y] else ".", end="")
        print()


if __name__ == "__main__":
    main()
