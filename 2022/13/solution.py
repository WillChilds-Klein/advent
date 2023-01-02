import json
import sys
import typing


def main():
    lines = list(map(str.strip, open(sys.argv[1]).readlines()))
    lines.append("")  # extra '' empty line to catch last packet pair
    pairs = []
    left, right = None, None
    for line in lines:
        if not line or (left is not None and right is not None):
            pairs.append((left, right))
            left, right = None, None
        elif left is None and right is None:
            left = json.loads(line)
        elif left is not None and right is None:
            right = json.loads(line)
        else:
            raise Exception("Malformed input")

    def compare(left: list, right: list) -> typing.Optional[bool]:
        for l, r in zip(left, right):
            if type(l) is not type(r):
                if type(l) is not list:
                    l = [l]
                if type(r) is not list:
                    r = [r]
            if type(l) == type(r) == list:
                res = compare(l, r)
                if res is not None:
                    return res
            elif type(l) == type(r) == int:
                if l < r:
                    return True
                elif l > r:
                    return False
            else:
                raise Exception(f"Unsupported input types {type(l)} and {type(r)}")
        if len(left) > len(right):
            return False
        if len(left) < len(right):
            return True
        return None

    count = 0
    for i, pair in enumerate(pairs):
        count += i + 1 if compare(pair[0], pair[1]) else 0
    print("PART 1:", count)


if __name__ == "__main__":
    main()
