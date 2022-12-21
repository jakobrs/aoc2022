# Largely based on orlp's solution: <https://github.com/orlp/aoc2022/blob/master/src/treap.rs>
# and the cp-algorithms page for treaps


import random
from typing import Tuple, Optional, Self
import sys


def gen_priority():
    return random.randint(0, 100000000)


class Node:
    priority: int
    count: int
    left: Optional[Self]
    right: Optional[Self]
    parent: Optional[Self]
    value: int

    def __init__(self, value: int) -> None:
        self.priority = gen_priority()
        self.count = 1
        self.left = None
        self.right = None
        self.parent = None
        self.value = value

    def update(self) -> None:
        count = 1
        if self.left is not None:
            count += self.left.count
            self.left.parent = self
        if self.right is not None:
            count += self.right.count
            self.right.parent = self
        self.count = count


class Treap:
    root: Optional[Node]

    def __init__(self) -> None:
        self.root = None

    def update(_self, node: Node) -> None:
        node.update()

    def split(self, rank: int) -> Tuple[Node, Node]:
        def go(node: Optional[Node], rank):
            if node is None:
                return None, None
            left_count = node.left.count if node.left is not None else 0
            if left_count >= rank:
                ll, lr = go(node.left, rank)
                node.left = lr
                node.update()
                return ll, node
            else:
                rl, rr = go(node.right, rank - left_count - 1)
                node.right = rl
                node.update()
                return node, rr

        return go(self.root, rank)

    def merge(_self, left: Optional[Node], right: Optional[Node]) -> Node:
        def go(left: Optional[Node], right: Optional[Node]) -> Node:
            if left is None:
                return right
            if right is None:
                return left

            if left.priority > right.priority:
                # Merge into left
                left.right = go(left.right, right)
                left.update()
                return left
            else:
                right.left = go(left, right.left)
                right.update()
                return right

        return go(left, right)

    def insert(self, rank: int, value: int) -> Node:
        left, right = self.split(rank)
        new_node = Node(value)
        left = self.merge(left, new_node)
        self.root = self.merge(left, right)
        self.root.parent = None

        return new_node

    def get(self, rank: int) -> Node:
        def go(node: Node, rank: int) -> Node:
            left_count = node.left.count if node.left is not None else 0
            if left_count == rank:
                return node
            elif left_count > rank:
                return go(node.left, rank)
            else:
                return go(node.right, rank - left_count - 1)

        return go(self.root, rank)

    def rank(_self, node: Node) -> int:
        rank = node.left.count if node.left is not None else 0
        cur = node.parent
        prev = node

        while cur is not None:
            if prev == cur.right:
                rank += cur.left.count if cur.left is not None else 0
                rank += 1
            cur, prev = cur.parent, cur

        return rank

    def remove(self, node: Node) -> Tuple[int, int]:
        rank = node.left.count if node.left is not None else 0
        cur = node.parent
        prev = node

        while cur is not None:
            cur.count -= 1
            if prev == cur.right:
                rank += cur.left.count if cur.left is not None else 0
                rank += 1
            cur, prev = cur.parent, cur

        merged = self.merge(node.left, node.right)
        if merged is not None:
            merged.parent = node.parent
        if node.parent is not None:
            if node.parent.left == node:
                node.parent.left = merged
            else:
                node.parent.right = merged
        else:
            self.root = merged

        return node.value, rank

    def __len__(self):
        return self.root.count


def run(numbers, multiplier, rounds):
    treap = Treap()
    nodes = [
        treap.insert(i, v * multiplier)
        for i, v in enumerate(numbers)
    ]

    n = len(nodes)

    for _ in range(rounds):
        for i, node in enumerate(nodes):
            value, rank = treap.remove(node)
            new_rank = (rank + value) % (n - 1)
            nodes[i] = treap.insert(new_rank, value)

    lst = [
        treap.get(i).value
        for i in range(n)
    ]

    zero_index = lst.index(0)
    return sum(lst[(zero_index + i*1000) % n] for i in range(1, 4))


def run_naive(numbers, multiplier, rounds):
    values = [(i, v * multiplier) for i, v in enumerate(numbers)]
    n = len(values)

    def find_by(pred):
        for j in range(n):
            k, v = values[j]
            if pred(k, v):
                return j

    for _ in range(rounds):
        for i in range(n):
            index = find_by(lambda k, _: k == i)
            _, value = values[index]
            values.pop(index)
            values.insert((index + value) % (n - 1), (i, value))

    zero_index = find_by(lambda _, v: v == 0)
    return sum(values[(zero_index + i*1000) % n][1] for i in range(1, 4))


numbers = [int(line) for line in sys.stdin]

print("Finished parsing input")
print(f"Part 1: {run(numbers, 1, 1)}")
print(f"Part 2: {run(numbers, 811589153, 10)}")
print("Naively:")
print(f"Part 1: {run_naive(numbers, 1, 1)}")
print(f"Part 2: {run_naive(numbers, 811589153, 10)}")
