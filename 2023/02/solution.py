import functools
import re
import sys


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

def game_power(game: list[dict[str,int]]) -> int:
    min_sets = {}
    for draw in game:
        for color, count in draw.items():
            if color not in min_sets or min_sets[color] < count:
                min_sets[color] = count
    return functools.reduce(lambda x, a: x * a, min_sets.values())

def main():
    lines = [l for l in sys.stdin.readlines()]
    rsum = 0
    for line in lines:
        game = parse_game(line.split(': ')[1])
        rsum += game_power(game)
    print(rsum)

if __name__ == '__main__':
    main()
