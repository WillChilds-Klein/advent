import sys


def main():
    lines = open(sys.argv[1]).readlines()
    instructions = []
    for line in lines:
        instruction = line.split(" ")
        instruction[0] = instruction[0].strip()
        if len(instruction) == 2:
            instruction[1] = int(instruction[1])
        instructions.append(instruction)
    cycle, signal_sum = 0, 0
    x, instruction_cycles = 1, 0
    instruction = [None]
    crt = [["." for _ in range(40)] for _ in range(6)]
    while len(instructions) > 0 or instruction_cycles > 0:
        # check cycle number, update signal_sum if applicable
        if (cycle + 20) % 40 == 0:
            signal_sum += cycle * x
        # either init case or time for the next instruction
        if instruction_cycles == 0:
            # do post-processing of last instruction
            if instruction[0] == "addx":
                x += instruction[1]
            # get the next instruction and set instruction_cycles accordingly
            instruction = instructions.pop(0) if instructions else [None]
            if instruction[0] == "noop":
                instruction_cycles = 1
            elif instruction[0] == "addx":
                instruction_cycles = 2
            else:
                raise Exception(f"Unrecognized op: {instruction[0]}")
        row = (cycle % (len(crt) * len(crt[0]))) // len(crt[0])
        pixel = cycle % len(crt[0])
        crt[row][pixel] = "#" if x - 1 <= pixel <= x + 1 else "."
        # one clock tick/cycle per iteration
        instruction_cycles -= 1
        cycle += 1
    print("PART 1:", signal_sum)
    print("PART 2:")
    draw_crt(crt)


def draw_crt(crt: list[list[str]]):
    for row in crt:
        print("".join(row))


if __name__ == "__main__":
    main()
