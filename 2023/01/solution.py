import sys


def main():
    lines = [l for l in sys.stdin.readlines()]
    rsum = 0
    for line in lines:
        first = int(list(filter(lambda c: c.isdigit(), line))[0])
        last = int(list(filter(lambda c: c.isdigit(), line[::-1]))[0])
        rsum += first*10 + last
    print(rsum)

if __name__ == '__main__':
    main()
