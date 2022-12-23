import sys

def main():
    lines = open(sys.argv[1]).readlines()
    trees = []
    for line in lines:
        trees.append(list(map(int, list(line.strip()))))
    count = 2 * len(trees) + 2 * len(trees[0]) - 4
    for i in range(1, len(trees)-1):
        for j in range(1, len(trees[0])-1):
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

#       30373
#       25512
#       65332
#       33549
#       35390

if __name__ == '__main__':
    main()
