import sys

def main():
    scores = {'A': 1, 'B': 2, 'C': 3, 'X': 1, 'Y': 2, 'Z': 3}
    lines = open(sys.argv[1]).readlines()
    rounds = [list(map(lambda y: scores[y], x.strip().split(' '))) for x in lines]
    def calc_score(other: int, me: int) -> int:
        if (me - other) in [-2, 1]:
            return 6 + me
        elif other == me:
            return 3 + me
        else:
            return 0 + me
    scores = map(lambda x: calc_score(x[0], x[1]), rounds)
    print("PART 1: " + str(sum(scores)))
    print("PART 2: ")

if __name__ == '__main__':
    main()
