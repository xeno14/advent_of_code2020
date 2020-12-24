

def parse_line(line: str):
    import re

    parts = re.findall(r"(.*)\(contains (.*)\)", line)
    ings = [s.strip() for s in parts[0][0].split()]
    allrs = [s.strip() for s in parts[0][1].split(",")]

    return (set(ings), set(allrs))


def main():
    lines = []
    # with open("input/day21-example.txt") as f:
    with open("input/day21.txt") as f:
        for line in f.readlines():
            ings, allrs = parse_line(line)
            lines.append([ings, allrs])
    
    # create a map from allergen to set of ingredients
    # take intersection for the same allergen
    allr_to_ings = dict()
    for ings, allrs in lines:
        for allr in allrs:
            if allr in allr_to_ings:
                allr_to_ings[allr] = allr_to_ings[allr] & ings
            else:
                allr_to_ings[allr] = set(ings)
    
    # now determin 1-to-1 map between ingredients and allergens
    allr_map = dict()
    update = True
    while update:
        update = False
        for allr in allr_to_ings:
            # ok, only 1 candidate. that guy is a dangerous ingredient!
            if len(allr_to_ings[allr]) == 1:
                ing = allr_to_ings[allr].pop()
                allr_map[allr] = ing
                update = True
                # remove the determined allergen from all others
                for k in allr_to_ings:
                    if ing in allr_to_ings[k]:
                        allr_to_ings[k].remove(ing)
    print(allr_map)

    ing_set = set(allr_map.values())
    ans = 0
    for ings, _ in lines:
        diff = ings - ing_set
        ans += len(diff)

    print(ans)

    # get canonical dangerous ingredients list
    sorted_keys = sorted(allr_map.keys())
    ans2 = ",".join([allr_map[k] for k in sorted_keys])
    print(ans2)


if __name__ == "__main__":
    main()