from __future__ import annotations

import dataclasses
import re
import sys


@dataclasses.dataclass
class Node:
    name: str
    parent: Node = None
    children: dict[str,Node] = dataclasses.field(default_factory=dict)
    is_dir: bool = True
    size: int = 0


def main():
    lines = map(str.strip, open(sys.argv[1]).readlines())
    root = Node('/')
    cwd = root
    for line in lines:
        name = re.search("([^ ]+)$", line).group(1)
        if line == '$ ls':
            continue
        elif line.startswith('$ cd '):
            if name == '/':
                cwd = root
            elif name == '..':
                cwd = cwd.parent if cwd.parent else root
            else:
                cwd = cwd.children[name]
        elif line.startswith('dir '):
            child = Node(name, parent=cwd)
            cwd.children[name] = child
        elif re.search('^\d+ ', line):
            size = int(re.search('^(\d+) ', line).group(1))
            child = Node(name, parent=cwd, is_dir=False, size=size)
            cwd.children[name] = child
        else:
            raise Exception('Unexpected format: ' + line)

    def set_dir_sizes(node: Node):
        if node is None or not node.is_dir:
            return
        for _, child in node.children.items():
            set_dir_sizes(child)
            node.size += child.size
    set_dir_sizes(root)

    def sum_sizes_under(node: Node, limit=100000) -> int:
        if node is None or not node.is_dir:
            return 0
        ret = node.size if node.size < limit else 0
        for _, child in node.children.items():
            ret += sum_sizes_under(child)
        return ret
    print("PART 1:", sum_sizes_under(root))

    unused = 70000000 - root.size
    target = 30000000 - unused
    if unused < 0:
        return
    candidate = 2**32-1 # simulate INT_MAX
    q = [root]
    while q:
        x = q.pop(0)
        if x.size > target and x.size < candidate:
            candidate = x.size
        for name, node in x.children.items():
            if node.is_dir:
                q.append(node)
    print("PART 2:", candidate)




if __name__ == '__main__':
    main()
