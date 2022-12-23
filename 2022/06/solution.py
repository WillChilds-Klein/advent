import sys

WINDOW_LENGTH = 4

def main():
    stream = list(str(open(sys.argv[1]).read()))
    window = []
    count = 0
    for i in range(len(stream)):
        count += 1
        if i < WINDOW_LENGTH:
            continue
        window = set(stream[i-WINDOW_LENGTH+1:i+1])
        if len(window) >= WINDOW_LENGTH:
            break
    print("PART 1:", count)

if __name__ == '__main__':
    main()
