import re
import sys
from typing import Dict, List, Tuple, Iterator


class Node:
    neighbours: List[int]
    pressure: int

    def __init__(self, neighbours: List[int], pressure: int) -> None:
        self.neighbours = neighbours
        self.pressure = pressure


def parse(input: str):
    vault_regex = re.compile(r"[A-Z]{2}")
    number_regex = re.compile(r"\d+")

    name_to_index: Dict[str, int] = dict()
    nodes_in_order: List[Tuple[List[str], int]] = []
    index = 0
    for line in input.splitlines():
        pressure = int(number_regex.findall(line)[0])

        names = vault_regex.findall(line)
        here = names[0]

        name_to_index[here] = index
        nodes_in_order.append((names[1:], pressure))

        index += 1

    nodes = []
    for neighbours, pressure in nodes_in_order:
        nodes.append(Node(
            neighbours=[name_to_index[name] for name in neighbours],
            pressure=pressure,
        ))

    return nodes, name_to_index["AA"]


def solve(nodes: List[Node], root: int) -> int:
    n = len(nodes)

    dist = [[float("inf")] * n for _ in range(n)]
    for i, node in enumerate(nodes):
        for neighbour in node.neighbours:
            dist[i][neighbour] = 1
        dist[i][i] = 0

    # Floyd-Warshall
    for k in range(n):
        for i in range(n):
            for j in range(n):
                if dist[i][j] > dist[i][k] + dist[k][j]:
                    dist[i][j] = dist[i][k] + dist[k][j]

    pressurised_valves = [
        i
        for i, node in enumerate(nodes)
        if node.pressure > 0
    ]

    n_pressurised = len(pressurised_valves)

    def brute_force(t: int, i: int, seen: int) -> int:
        best = 0

        for seen_j, j in enumerate(pressurised_valves):
            if not (seen & (1 << seen_j)):
                new_t = t + dist[i][j] + 1
                if new_t <= 30:
                    a = brute_force(new_t, j, seen | (1 << seen_j)) + \
                        (30 - new_t) * nodes[j].pressure

                    if a > best:
                        best = a

        return best

    return brute_force(0, root, 0)


def main():
    input = sys.stdin.read()
    nodes, root = parse(input)
    part1 = solve(nodes, root)
    print(part1)


main()
