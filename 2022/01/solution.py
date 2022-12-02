import sys

def main():
    lines = open(sys.argv[1]).read()
    counts = [sum([int(i) for i in j.split('\n')]) for j in lines.split('\n\n')]
    print("PART 1: " + str(max(counts)))
    top_three = sum(sorted(counts)[-3:])
    print("PART 2: " + str(top_three))

if __name__ == '__main__':
    main()
