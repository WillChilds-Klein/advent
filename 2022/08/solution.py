import sys


def main():
    lines = open(sys.argv[1]).readlines()
    trees = []
    for line in lines:
        trees.append(list(map(int, list(line.strip()))))
    count = 2 * len(trees) + 2 * len(trees[0]) - 4
    for i in range(1, len(trees) - 1):
        for j in range(1, len(trees[0]) - 1):
            height = trees[i][j]
            visible = True
            angle_count = 0
            for k in range(0, len(trees)):
                if k == i:
                    if visible:
                        angle_count += 1
                    visible = True
                elif trees[k][j] >= height:
                    visible = False
            if visible:
                angle_count += 1
            visible = True
            for k in range(0, len(trees[0])):
                if k == j:
                    if visible:
                        angle_count += 1
                    visible = True
                elif trees[i][k] >= height:
                    visible = False
            if visible:
                angle_count += 1
            count += 1 if angle_count > 0 else 0
    print("PART 1:", count)

    max_score = -1
    for i in range(1, len(trees) - 1):
        for j in range(1, len(trees[0]) - 1):
            height = trees[i][j]
            score = 1
            # looking left
            direction_score = 0
            for k in range(i - 1, -1, -1):
                direction_score += 1
                if trees[k][j] >= height:
                    break
            score *= direction_score
            # looking right
            direction_score = 0
            for k in range(i + 1, len(trees)):
                direction_score += 1
                if trees[k][j] >= height:
                    break
            score *= direction_score
            # looking up
            direction_score = 0
            for k in range(j - 1, -1, -1):
                direction_score += 1
                if trees[i][k] >= height:
                    break
            score *= direction_score
            # looking down
            direction_score = 0
            for k in range(j + 1, len(trees[0])):
                direction_score += 1
                if trees[i][k] >= height:
                    break
            score *= direction_score

            if score > max_score:
                max_score = score

    print("PART 2:", max_score)


if __name__ == "__main__":
    main()
