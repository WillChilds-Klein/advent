import sys

def main():
    vals = {'A': 1, 'B': 2, 'C': 3, 'X': 1, 'Y': 2, 'Z': 3}
    lines = open(sys.argv[1]).readlines()
    rounds = [list(map(lambda y: vals[y], x.strip().split(' '))) for x in lines]
    def calc_score(other: int, me: int) -> int:
        if (me - other) in [-2, 1]:
            return 6 + me
        elif other == me:
            return 3 + me
        else:
            return 0 + me
    scores = map(lambda x: calc_score(x[0], x[1]), rounds)
    print("PART 1: " + str(sum(scores)))
    lose, draw, win = vals['X'], vals['Y'], vals['Z']
    def get_move(other: int, outcome: int) -> int:
        if outcome == lose:
            return (other + 1) % 3 + 1
        elif outcome == draw:
            return other
        elif outcome == win:
            return other % 3 + 1
        assert False
    scores = [calc_score(x, get_move(x, y)) for x, y in rounds]
    print("PART 2: " + str(sum(scores)))

if __name__ == '__main__':
    main()
