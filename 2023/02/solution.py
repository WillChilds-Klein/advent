import re
import sys

TARGET = {'red': 12, 'green': 13, 'blue': 14}


def parse_game(line: str) -> list[dict[str,int]]:
    ret = []
    draw_sets = map(str.strip, line.split(';'))
    for draw_set in draw_sets:
        draw = {}
        for draw_strs in draw_set.split(', '):
            r = re.search('([\d]+) ([a-z]+)', draw_strs)
            count = int(r.group(1))
            color = r.group(2)
            draw[color] = count
        ret.append(draw)
    return ret

def is_valid(draw: dict[str,int]) -> bool:
    for color, count in draw.items():
        if color not in TARGET or TARGET[color] < count:
            return False
    return True

def main():
    lines = [l for l in sys.stdin.readlines()]
    rsum = 0
    for line in lines:
        idx = int(line.split(': ')[0].split(' ')[1])
        game = parse_game(line.split(': ')[1])
        if all(map(is_valid, game)):
            rsum += idx
    print(rsum)

if __name__ == '__main__':
    main()
