import collections
import re


def parse_line(line: str):
    a = re.findall(r"([a-z]+ [a-z]+) bags contain", line)
    b = re.findall(r"\d+ ([a-z]+ [a-z]+) bag", line)
    return a[0], b


def build_graph(lines: list):
    g = dict()
    for line in lines:
        u, l = parse_line(line)
        assert(u!=g)
        g[u] = l
    return g


def main():
    # filename = "input/day7-example.txt"
    filename = "input/day7.txt"
    with open(filename) as f:
        g = build_graph(f.readlines())
    # print(g)

    ok = set()  # reachable to shiny gold
    goal = "shiny gold"

    # returns true if the node is reachable to the goal
    def rec(u) -> bool:
        for v in g[u]:
            if v in ok or v == goal or rec(v):
                ok.add(u)
                return True
        return False
    for u in g:
        rec(u)
    print(len(ok))


if __name__ == "__main__":
    main()