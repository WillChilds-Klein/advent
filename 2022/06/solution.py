import sys

PACKET_WINDOW_LENGTH = 4
MESSAGE_WINDOW_LENGTH = 14

def main():
    stream = list(str(open(sys.argv[1]).read()))
    print("PART 1:", marker_index(stream, PACKET_WINDOW_LENGTH))
    print("PART 2:", marker_index(stream, MESSAGE_WINDOW_LENGTH))

def marker_index(stream: list[str], length: int) -> int:
    count = 0
    for i in range(len(stream)):
        count += 1
        if i < length:
            continue
        window = set(stream[i-length+1:i+1])
        if len(window) >= length:
            break
    return count


if __name__ == '__main__':
    main()
