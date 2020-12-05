def iter_items(stream):
    while line := stream.readline():
        if len == "\n":
            continue
        pieces = [line.strip()]
        while line := stream.readline():
            if line == "\n":
                break
            pieces.append(line.strip())
        if len(pieces) > 0:
            yield " ".join(pieces)


def is_valid(passport: str):
    required = {
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        # optional=>"cid"
    }
    for item in passport.split():
        field = item.split(":")[0]
        if field in required:
            required.remove(field)
    return len(required) == 0


def main():
    # f = open("input/day4-example.txt", "r")
    f = open("input/day4.txt", "r")

    ans = sum(map(is_valid, iter_items(f)))
    print(ans)

    f.close()


if __name__ == "__main__":
    main()
