import dataclasses
import re
import sys
import typing


@dataclasses.dataclass
class Point:
    x: int
    y: int

    def __hash__(self) -> int:
        return hash((self.x, self.y))


def main():
    lines = list(map(str.strip, open(sys.argv[1]).readlines()))
    beacons, mappings = set(), set()
    for line in lines:
        pattern = "Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)"
        sx, sy, bx, by = re.search(pattern, line).groups()
        s, b = Point(int(sx), int(sy)), Point(int(bx), int(by))
        beacons.add(b)
        mappings.add((s, b))
    min_x = min(*[s.x - distance(s, b) for s, b in mappings])
    max_x = max(*[s.x + distance(s, b) for s, b in mappings])
    min_y = min(*[s.y - distance(s, b) for s, b in mappings])
    max_y = max(*[s.y + distance(s, b) for s, b in mappings])
    covered_count = 0
    no_beacon_row = 2000000
    print(min_x, max_x+1)
    for i in range(min_x, max_x+1):
        p = Point(i, no_beacon_row)
        if p in beacons:
            continue
        for s, b in mappings:
            if distance(s, p) <= distance(s, b):
                covered_count += 1
                break
        else:
            continue

    print("PART 1:", covered_count)

    min_coord, max_coord = 0, 4000000
    bounding_box_points = set()
    for s, b in mappings:
        d = distance(s, b)
        for i in range(d+1):
            bounding_box_points.add(Point(s.x+i, s.y+(d-i)))
            bounding_box_points.add(Point(s.x+(d-i), s.y-i))
            bounding_box_points.add(Point(s.x-i, s.y-(d-i))
            bounding_box_points.add(Point(s.x-(d-i), s.y+i))

def distance(a: Point, b: Point) -> int:
    return abs(a.x - b.x) + abs(a.y - b.y)


if __name__ == "__main__":
    main()
