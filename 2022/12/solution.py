import sys

INT_MAX = 2 ** 32 - 1

Point = tuple[int,int]

def main():
    sys.setrecursionlimit(10000)
    lines = open(sys.argv[1]).readlines()
    hmap = [[c for c in line.strip() ] for line in lines]

    s, e = None, None
    for i in range(len(hmap)):
        for j in range(len(hmap[0])):
            if hmap[i][j] == 'S':
                s = (i, j)
                hmap[i][j] = 'a'
            elif hmap[i][j] == 'E':
                e = (i, j)
                hmap[i][j] = 'z'
            hmap[i][j] = ord(hmap[i][j]) - ord('a')

    def is_in_bounds(p: Point) -> bool:
        return p[0] >= 0 and p[0] < len(hmap) and p[1] >= 0 and p[1] < len(hmap[0])

    def is_valid_height(p: Point, candidate: Point) -> bool:
        return hmap[candidate[0]][candidate[1]] - hmap[p[0]][p[1]] <= 1

    edges = set()
    for i in range(len(hmap)):
        for j in range(len(hmap[0])):
            p = (i, j)
            candidates = [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)]
            candidates = [c for c in candidates if is_in_bounds(c)]
            candidates = [c for c in candidates if is_valid_height(p, c)]
            edges.update([(p, c) for c in candidates])

    # cf. https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm
    # cf. https://www.notion.so/willck/Week-13-Graph-Algorithms-pt-2-b38e8b93085e4526a86303dc44172d37#62f4012220c84f748db5a0268b3b2aef
    sssp = [[INT_MAX for _ in range(len(hmap[0]))] for _ in range(len(hmap))]
    sssp[s[0]][s[1]] = 0
    for _ in range(len(hmap)*len(hmap[0])):
        for u, v in edges:
            candidate_distance = sssp[u[0]][u[1]] + 1
            if candidate_distance < sssp[v[0]][v[1]]:
                sssp[v[0]][v[1]] = candidate_distance

    print("PART 1:", sssp[e[0]][e[1]])

    edges = set()
    vertices = set()
    for i in range(len(hmap)):
        for j in range(len(hmap[0])):
            p = (i, j)
            vertices.add(p)
            candidates = [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)]
            candidates = [c for c in candidates if is_in_bounds(c)]
            # NOTE: swap p and candidate params in is_valid_height call below
            candidates = [c for c in candidates if is_valid_height(c, p)]
            edges.update([(p, c) for c in candidates])

    sssp = [[INT_MAX for _ in range(len(hmap[0]))] for _ in range(len(hmap))]
    sssp[e[0]][e[1]] = 0
    for _ in range(len(hmap)*len(hmap[0])):
        for u, v in edges:
            candidate_distance = sssp[u[0]][u[1]] + 1
            if candidate_distance < sssp[v[0]][v[1]]:
                sssp[v[0]][v[1]] = candidate_distance

    print("PART 2:", min(*[sssp[x][y] for x, y in vertices if hmap[x][y] == 0]))


if __name__ == '__main__':
    main()
