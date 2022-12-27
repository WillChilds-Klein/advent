import collections
import dataclasses
import functools
import inspect
import re
import sys
import typing

ROUNDS = 20

INSPECTIONS = collections.Counter()


@dataclasses.dataclass
class Monkey:
    m_id: int
    operation_strs: list[str]
    test: list[int]
    # test: typing.Callable[int, int]
    items: list[int]

    # return is list[(value, dst)]
    def turn(self) -> list[tuple[int, int]]:
        ret = []
        for item in self.items:
            worry = self.operation(item) // 3
            INSPECTIONS.update([self.m_id])
            # dst = self.test(worry)
            true_dst, modulus, false_dst = self.test
            dst = true_dst if worry % modulus == 0 else false_dst
            ret.append((worry, dst))
        self.items.clear()
        return ret

    def operation(self, item_worry: int) -> int:
        if not self.operation_strs:
            raise Exception("Need to initialize operation_strs")
        l_arg, op, r_arg = self.operation_strs
        l_arg = item_worry if l_arg == "old" else int(l_arg)
        r_arg = item_worry if r_arg == "old" else int(r_arg)
        if op == "+":
            return l_arg + r_arg
        elif op == "*":
            return l_arg * r_arg
        else:
            raise(f"Invalid op: {op}")


def main():
    lines = open(sys.argv[1]).readlines()
    monkeys = []
    m_id, items, operation_strs, test = None, None, None, None
    line = lines.pop(0)
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
            line = lines.pop(0).strip()
            true_dst = int(re.search(" (\d+)$", line).group(1))
            line = lines.pop(0).strip()
            false_dst = int(re.search(" (\d+)$", line).group(1))
            test = true_dst, modulus, false_dst
            monkeys.append(
                Monkey(m_id=m_id, items=items, operation_strs=operation_strs, test=test)
            )
        elif line == "":
            pass
        else:
            raise Exception(f"Invalid line: {line}")
        line = lines.pop(0).strip() if lines else None
    for _ in range(ROUNDS):
        for monkey in monkeys:
            t = monkey.turn()
            for item, dst in t:
                monkeys[dst].items.append(item)
    print("PART 1", functools.reduce(lambda x, y: x * y[1], INSPECTIONS.most_common(2), 1))


if __name__ == "__main__":
    main()
