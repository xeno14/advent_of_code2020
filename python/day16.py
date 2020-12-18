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


def invalid_for_any(rules, x) -> bool:
    for ranges in rules.values():
        for (lo, hi) in ranges:
            if lo <= x <= hi:
                return False 
    return True


def valid_ticket(rules, ticket) -> bool:
    for x in ticket:
        if invalid_for_any(rules, x):
            return False
    return True


def possible_pos(rules, field, tickets) -> int:
    lo0, hi0 = rules[field][0]
    lo1, hi1 = rules[field][1]
    # position of a given field
    res = []
    for i in range(len(rules)):
        values = [ticket[i] for ticket in tickets]
        if all(lo0 <= x <= hi0 or lo1 <= x <= hi1 for x in values):
            res.append(i)
    return res


def main():
    # with open("input/day16.txt") as f:
    # # with open("input/day16-example.txt") as f:
    #     rules, mine, tickets= parse(f.read())
    # pprint.pprint(rules)

    # # part1
    # ans = 0
    # for ticket in tickets:
    #     for x in ticket:
    #         if invalid_for_any(rules, x):
    #             ans += x
    # print(ans)

    # part2
    with open("input/day16.txt") as f:
    # with open("input/day16-example2.txt") as f:
        rules, mine, tickets= parse(f.read())

    tickets = [ticket for ticket in tickets if valid_ticket(rules, ticket)]

    # possible position for each field
    # e.g.
    # {'class': [1, 2], 'row': [0, 1, 2], 'seat': [2]}
    options = {field: set(possible_pos(rules, field, tickets)) for field in rules}
    pprint.pprint(options)

    # field -> pos can be determened if there is an unique option
    # similar to topological sort algorithm
    mapping = {}
    while len(mapping) < len(rules):
        for field, pos in options.items():
            if len(pos) == 1:  # unique!
                idx = pos.pop()
                mapping[field] = idx
                # remove i from all the options
                for v in options.values():
                    if idx in v:
                        v.remove(idx)
    pprint.pprint(mapping)

    import math
    ans = math.prod(
        [mine[i] for field, i in mapping.items() if field.startswith("departure")], start=1)
    print(ans)


if __name__ == "__main__":
    main()
