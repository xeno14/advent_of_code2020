use aoc::read_lines;

struct Input {
    target: u32,
    times: Vec<u32>,
}

fn read_input(filename: &str) -> Input {
    let lines: Vec<String> = read_lines(filename).map(|line| line.unwrap()).collect();
    let target: u32 = lines[0].parse().unwrap();
    let mut times: Vec<u32> = Vec::new();
    for x in lines[1].split(',') {
        let n = match x.parse::<u32>() {
            Ok(n) => n,
            Err(_) => 0,
        };
        times.push(n);
    }

    Input { target, times }
}

fn parse_line(line: &str) -> Vec<(i64, i64)> {
    line.split(',')
        .enumerate()
        .map(|(i, c)| match c.parse::<i64>() {
            Ok(x) => Some((x, -(i as i64))),
            Err(_) => None,
        })
        .filter_map(|x| x)
        .collect()
}

/// solves ax + by = gcd(a, b). returns (x, y, d) where
/// (x, y) is a solution and d = gcd(a, b)
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    // dx + 0y = d
    if b == 0 {
        (1, 0, a)
    }
    // how to get a solution in a recursive way:
    // let's say we have a solution (s, t) for bs + rt = d where r=a%b.
    // substituting a=qb+r to ax+by=d gives:
    // (qb + r)x + by = d => b(qx + y) + rx = d
    // then s = qx + y, t = x  =>  x = t, y = s - qx
    else {
        let (s, t, d) = ext_gcd(b, a % b);
        let x = t;
        let y = s - (a / b) * x;
        (x, y, d)
    }
}

/// gives x (mod mn) which satisifies x=a (mod m), x=b (mon n) where m, n are
/// coprime
fn solve_mod_eq(a: i64, m: i64, b: i64, n: i64) -> i64 {
    // using Chinese remainder theorem
    // obtain u, v such that mu + nv = 1
    let (u, v, d) = ext_gcd(m, n);
    if d != 1 {
        panic!("m and n are not coprime!");
    }
    let mn = m * n;
    let x = (a * n * v + b * m * u) % mn;
    if x > 0 {
        x
    } else {
        x + mn
    }
}

fn solve_part2(vec: Vec<(i64, i64)>) -> i64 {
    // repeatedly solve mod equation
    let (mut m, mut a) = vec[0];
    for i in 1..vec.len() {
        let (n, b) = vec[i];
        let x = solve_mod_eq(a, m, b, n);
        a = x;
        m = m * n;
    }
    return a;
}

fn main() {
    // let filename = "input/day13-example.txt";
    let filename = "input/day13.txt";
    let input = read_input(filename);

    let t = input
        .times
        .iter()
        .filter(|&t| *t > 0)
        .min_by_key(|&t| (input.target / t + 1) * t)
        .unwrap();
    let ans = t * (t - input.target % t);
    println!("{}", ans);
    // part2
    assert_eq!((3, -11, 3), ext_gcd(111, 30));
    assert_eq!(77, solve_mod_eq(0, 7, -1, 13));
    let vec = vec![(17, 0), (13, -2), (19, -3)];
    assert_eq!(3417, solve_part2(vec));
    let vec = parse_line("17,x,13,19");
    assert_eq!(3417, solve_part2(vec));
    let vec = parse_line("67,7,59,61");
    assert_eq!(754018, solve_part2(vec));
    let vec = parse_line("67,7,x,59,61");
    assert_eq!(1261476, solve_part2(vec));
    let vec = parse_line("1789,37,47,1889");
    assert_eq!(1202161486, solve_part2(vec));

    // umm... it overflows....
    // let line = read_lines(filename).skip(1).next().unwrap().unwrap();
    // let vec = parse_line(&line);
    // let ans = solve_part2(vec);
    // println!("{}", ans);
}
