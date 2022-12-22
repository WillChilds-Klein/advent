import sys

def main():
    lines = open(sys.argv[1]).readlines()
    pairs = [[[int(s) for s in p.split('-')] for p in l.split(',')] for l in lines]
    ctr = 0
    for a, b in pairs:
        if a[0] <= b[0] and a[1] >= b[1] or b[0] <= a[0] and b[1] >= a[1]:
            ctr += 1
    print('PART 1:', str(ctr))
    ctr = 0
    for a, b in pairs:
        if a[1] >= b[0] and a[0] <= b[1] or b[1] >= a[0] and b[0] <= a[1]:
            ctr += 1
    print('PART 2:', str(ctr))



if __name__ == '__main__':
    main()
