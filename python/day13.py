from typing import Tuple, List


def ext_gcd(a: int, b: int) -> Tuple[int, int, int]:
    """extended Euclidean algorithm
    """
    if b == 0:
        return (1, 0, a)
    else:
        (s, t, d) = ext_gcd(b, a % b)
        x = t
        y = s-(a//b)*x
        return (x, y, d)


def solve_mod_eq(a:int, m:int, b:int, n: int)->int:
    """gives a solution x for
        x = a (mod m)
        x = b (mod n)
    where m, n are coprime using Chinese remainder theorem
    """
    u, v, d = ext_gcd(m, n)
    assert(d==1)

    mn = m*n
    x = (a*n*v + b*m*u) % mn
    if x < 0:
        x += mn
    return x


def parse_line(line: str) -> List[Tuple[int, int]]:
    """17,x,13,19  ->  [(17, 0), (13, 2), (19, 3)]
    """
    res = []
    for i, s in enumerate(line.strip().split(",")):
        if s == "x":
            continue
        res.append((int(s), -i))
    return res


def solve(l) -> int:
    """repeatedly apply Chinese remainder theorem

    pre: all the inputs are coprime
    """
    m, a = l[0]
    for n, b in l[1:]:
        x = solve_mod_eq(a, m, b, n)
        a = x
        m = m*n
    return a


def main():
    assert((3, -11, 3) == ext_gcd(111, 30))
    assert(77 == solve_mod_eq(0, 7, -1, 13))
    assert(3417 == solve(parse_line("17,x,13,19")))
    assert(1202161486 == solve(parse_line("1789,37,47,1889")))

    with open("input/day13.txt") as f:
        lines = f.readlines()
        l = parse_line(lines[1])
    ans = solve(l)
    print(ans)


if __name__ == "__main__":
    main()

