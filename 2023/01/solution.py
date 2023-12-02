import sys

numbers = {'one': 1,
           'two': 2,
           'three': 3,
           'four': 4,
           'five': 5,
           'six': 6,
           'seven': 7,
           'eight': 8,
           'nine': 9,
}

def is_digit(s: str) -> bool:
    if len(s) == 1 and s[0].isdigit():
        return True
    if s in numbers.keys():
        return True
    return False

def convert_digit(s: str) -> int:
    """ assumes that |s| is a digit """
    if len(s) == 1 and s[0].isdigit():
        return int(s[0])
    return numbers[s]

def main():
    lines = [l for l in sys.stdin.readlines()]
    rsum = 0
    for line in lines:
        digits = []
        for i in range(len(line)+1):
            for j in range(i, len(line)+1):
                if is_digit(line[i:j+1]):
                    digits.append(convert_digit(line[i:j+1]))
        rsum += 10*digits[0] + digits[-1]
    print(rsum)

if __name__ == '__main__':
    main()
