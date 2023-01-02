import functools
import json
import operator
import secrets
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
    for i, (l, r) in enumerate(pairs):
        count += i + 1 if compare(l, r) else 0
    print("PART 1:", count)

    # cf. https://github.com/WillChilds-Klein/pysort/blob/master/sort.py#L47
    def qsort(pkts: list) -> list:
        if pkts == []:
            return pkts
        return qsort([pkt for pkt in pkts[1:] if compare(pkt, pkts[0])]) + pkts[:1] + \
               qsort([pkt for pkt in pkts[1:] if not compare(pkt, pkts[0])])

    packets = []
    for pair in pairs:
        packets.extend(pair)
    dividers = [[[2]], [[6]]]
    packets.extend(list(dividers))
    packets = qsort(packets)
    divider_idxs = []
    for i, packet in enumerate(packets):
        if packet in dividers:
            divider_idxs.append(i + 1)
    print("PART 2:", functools.reduce(operator.mul, divider_idxs, 1))


if __name__ == "__main__":
    main()
