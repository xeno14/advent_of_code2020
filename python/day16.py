import pprint


def parse(s):
    parts = s.split('\n\n')

    # rules
    rule = dict()
    for line in parts[0].split('\n'):
        field, ranges = line.split(":")
        rule[field] = []
        for rng in ranges.split("or"):
            lo, hi = rng.split("-")
            rule[field].append((int(lo), int(hi)))
    
    # my ticket
    mine = [int(x) for x in parts[1].split("\n")[1].split(",")]

    # nearby tickets
    tickets = []
    for line in parts[2].split("\n")[1:]:
        t = [int(x) for x in line.split(",")]
        tickets.append(t)

    return rule, mine, tickets


def invalid_for_any(rule, x) -> bool:
    for ranges in rule.values():
        for (lo, hi) in ranges:
            if lo <= x <= hi:
                return False 
    return True

def main():
    with open("input/day16.txt") as f:
    # with open("input/day16-example.txt") as f:
        rule, mine, tickets= parse(f.read())
    pprint.pprint(rule)
    # pprint.pprint(mine)
    # pprint.pprint(tickets)

    # part1
    ans = 0
    for ticket in tickets:
        for x in ticket:
            if invalid_for_any(rule, x):
                ans += x
    print(ans)


if __name__ == "__main__":
    main()