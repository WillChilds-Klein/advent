import collections
import copy
import dataclasses
import functools
import math
import re
import sys


INSPECTIONS = collections.Counter()


@dataclasses.dataclass
class Monkey:
    m_id: int
    operation_strs: list[str]
    test: list[int]
    items: list[int]
    lcm: int = -1

    # return is list[(value, dst)]
    def turn(self, worry_factor=1) -> list[tuple[int, int]]:
        ret = []
        for item in self.items:
            true_dst, modulus, false_dst = self.test
            worry = self.operation(item, modulus) // worry_factor
            INSPECTIONS.update([self.m_id])
            is_divisible = worry % modulus == 0
            ret.append((worry, true_dst if is_divisible else false_dst))
        self.items.clear()
        return ret

    def operation(self, item_worry: int, modulus: int) -> int:
        if not self.operation_strs:
            raise Exception("Need to initialize operation_strs")
        l_arg, op, r_arg = self.operation_strs
        l_arg = item_worry if l_arg == "old" else int(l_arg)
        r_arg = item_worry if r_arg == "old" else int(r_arg)
        if op == "+":
            return (l_arg + r_arg) % self.lcm
        elif op == "*":
            return modular_multiply(l_arg, r_arg, self.lcm)
        else:
            raise (f"Invalid op: {op}")


# cf. https://en.wikipedia.org/wiki/Modular_arithmetic#Example_implementations
def modular_multiply(a: int, b: int, m: int) -> int:
    if not ((a | b) & (0xFFFFFFFF << 32)):
        return (a * b) % m
    d = 0
    mp2 = m >> 1
    if a >= m:
        a %= m
    if b >= m:
        b %= m
    for i in range(64):
        d = (d << 1) - m if d > mp2 else d << 1
        if a & 0x8000000000000000:
            d += b
        if d >= m:
            d -= m
        a <<= 1
    return d


def main():
    lines = open(sys.argv[1]).readlines()
    original_monkeys = []
    m_id, items, operation_strs, test = None, None, None, None
    line = lines.pop(0)
    moduli = []
    while lines:
        if line.startswith("Monkey "):
            m_id = int(re.search(" (\d)+:$", line).group(1))
        elif line.startswith("Starting items:"):
            items = list(map(int, re.search(" ([\d ,]+)$", line).group(1).split(", ")))
        elif line.startswith("Operation:"):
            pattern = "new = ([a-z0-9]+) (\+|\*) ([a-z0-9]+)$"
            operation_strs = re.search(pattern, line).groups()
        elif line.startswith("Test:"):
            modulus = int(re.search(" (\d+)$", line).group(1))
            moduli.append(modulus)
            line = lines.pop(0).strip()
            true_dst = int(re.search(" (\d+)$", line).group(1))
            line = lines.pop(0).strip()
            false_dst = int(re.search(" (\d+)$", line).group(1))
            test = true_dst, modulus, false_dst
            original_monkeys.append(
                Monkey(m_id=m_id, items=items, operation_strs=operation_strs, test=test)
            )
        elif line == "":
            pass
        else:
            raise Exception(f"Invalid line: {line}")
        line = lines.pop(0).strip() if lines else None

    # compute the LCM across all moduli and set it on each monkey
    product = abs(functools.reduce(lambda x, y: x * y, moduli, 1))
    gcd = functools.reduce(math.gcd, moduli, moduli[0])
    lcm = product // gcd
    for monkey in original_monkeys:
        monkey.lcm = lcm

    monkeys = copy.deepcopy(original_monkeys)
    INSPECTIONS.clear()
    for _ in range(20):
        for monkey in monkeys:
            t = monkey.turn(worry_factor=3)
            for item, dst in t:
                monkeys[dst].items.append(item)
    print(
        "PART 1", functools.reduce(lambda x, y: x * y[1], INSPECTIONS.most_common(2), 1)
    )

    monkeys = copy.deepcopy(original_monkeys)
    INSPECTIONS.clear()
    for _ in range(10000):
        for monkey in monkeys:
            t = monkey.turn(worry_factor=1)
            for item, dst in t:
                monkeys[dst].items.append(item)
    print(
        "PART 2", functools.reduce(lambda x, y: x * y[1], INSPECTIONS.most_common(2), 1)
    )


if __name__ == "__main__":
    main()
