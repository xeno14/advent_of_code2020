from collections import deque
import itertools

def read_file(filename: str):
    with open(filename) as f:
        p1, p2 = f.read().strip().split("\n\n")
    p1 = [int(s) for s in p1.split("\n")[1:]]
    p2 = [int(s) for s in p2.split("\n")[1:]]
    return p1, p2


def part1(p1, p2):
    p1 = deque(p1)
    p2 = deque(p2)
    print(p1, p2)

    while p1 and p2:
        x = p1.popleft()
        y = p2.popleft()
        if x > y:
            p1.append(x)
            p1.append(y)
        else:
            p2.append(y)
            p2.append(x)

    winner = p1 if p1 else p2
    weight = len(winner)
    ans = 0
    for x in winner:
        ans += weight * x
        weight -= 1
    print(ans)


def score(deck) -> int:
    weight = len(deck)
    res = 0
    for x in deck:
        res += weight * x
        weight -= 1
    return res


PLAYER1 = 1
PLAYER2 = 2

def do_round(p1: deque, p2: deque):
    """returns (player, deck)
    """
    x = p1.popleft()
    y = p2.popleft()

    if len(p1) >= x and len(p2) >= y:
        # make copy
        q1 = deque(itertools.islice(p1, x))
        q2 = deque(itertools.islice(p2, y))
        winner, _ = do_game(q1, q2)
    else:
        winner = PLAYER1 if x > y else PLAYER2
    
    if winner == PLAYER1:
        p1.append(x)
        p1.append(y)
        deck = p1
    else:
        p2.append(y)
        p2.append(x)
        deck = p2

    return winner, deck


def do_game(p1: deque, p2: deque):
    tot = len(p1) + len(p2)
    hist = set()
 
    while True:
        h = (tuple(p1), tuple(p2))
        if h in hist:
            return PLAYER1, p1
        hist.add(h)

        winner, deck = do_round(p1, p2)

        if len(deck) == tot:
            break
    return winner, deck

def main():
    p1, p2 = read_file("input/day22.txt")
    # p1, p2 = read_file("input/day22-example.txt")
    # p1, p2 = read_file("input/day22-example2.txt")

    # part1(p1, p2)
    from collections import deque
    p1 = deque(p1)
    p2 = deque(p2)
    tot = len(p1) + len(p2)

    _, deck = do_game(p1, p2)
    print(deck)
    ans = score(deck)
    print(ans)


if __name__ == "__main__":
    main()
