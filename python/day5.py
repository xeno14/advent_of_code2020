

def parse(s: str):
    row = int(s[0:7].replace("F", "0").replace("B", "1"), 2)
    col = int(s[7:].replace("L", "0").replace("R", "1"), 2)
    return (row, col)


def main():
    assert (70, 7) == parse("BFFFBBFRRR")
    assert (14, 7) == parse("FFFBBBFRRR")

    ans = 0
    ids = []
    for line in open("input/day5.txt").readlines():
        r, c = parse(line)
        i = r*8+c
        ids.append(i)
        ans = max(ans, i)
    print(ans)

    ids = set(ids)
    for i in range(ans):
        if i not in ids and i+1 in ids and i-1 in ids:
            print(i)


if __name__ == "__main__":
    main()
