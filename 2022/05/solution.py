import sys
import re
import typing


def main():
    lines = open(sys.argv[1]).readlines()
    num_line = next(filter(lambda x: x.startswith(" 1"), lines))
    stack_count = int(re.search(" [\d]+ $", num_line)[0])
    stack_lines = []
    for line in lines:
        if not line.startswith(" 1"):
            stack_lines.append(line)
        else:
            break
    stacks = []
    for line in stack_lines:
        stacks.append(re.findall("\[\w\]|    ", line))
    stacks = normalize_stacks(stacks)
    for line in lines:
        if not line.startswith("move"):
            continue
        c, f, t = map(
            int, re.search("move (\d+) from (\d+) to (\d+)", line).group(1, 2, 3)
        )
        for _ in range(c):
            if stacks[f - 1]:
                stacks[t - 1].append(stacks[f - 1].pop())
    tops = [stack[-1] for stack in stacks]
    print("PART 1:", "".join(tops))


def normalize_stacks(stacks: list[list[str]]) -> list[list[typing.Optional[str]]]:
    normalized = [
        [None if crate == "    " else re.search("\w", crate)[0] for crate in stack]
        for stack in stacks
    ]
    transposed = [
        [normalized[j][i] for j in range(len(normalized))]
        for i in range(len(normalized[0]))
    ]
    for stack in transposed:  # reverse stacks so bottom crate is in 0th position
        stack.reverse()
    transposed = [[x for x in stack if x is not None] for stack in transposed]
    return transposed


if __name__ == "__main__":
    main()
