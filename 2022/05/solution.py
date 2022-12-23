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
    raw_stacks = []
    for line in stack_lines:
        raw_stacks.append(re.findall("\[\w\]|    ", line))
    moves = []
    for line in lines:
        if not line.startswith("move"):
            continue
        c, f, t = map(
            int, re.search("move (\d+) from (\d+) to (\d+)", line).group(1, 2, 3)
        )
        moves.append([c, f, t])
    stacks = normalize_stacks(raw_stacks)
    move_crates_individually(stacks, moves)
    print("PART 1:", "".join([stack[-1] for stack in stacks]))
    stacks = normalize_stacks(raw_stacks)
    move_crates_batch(stacks, moves)
    print("PART 2:", "".join([stack[-1] for stack in stacks]))


def move_crates_individually(stacks: list[list[str]], moves: list[list[int]]):
    for c, f, t in moves:
        for _ in range(c):
            stacks[t - 1].append(stacks[f - 1].pop())


def move_crates_batch(stacks: list[list[str]], moves: list[list[int]]):
    for c, f, t in moves:
        stacks[t - 1].extend(stacks[f - 1][(-1 * c) :])
        del stacks[f - 1][(-1 * c) :]


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
