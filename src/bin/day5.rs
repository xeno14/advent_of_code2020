fn main() {
    assert_eq!(567, parse("BFFFBBFRRR"));
    assert_eq!(119, parse("FFFBBBFRRR"));
    assert_eq!(820, parse("BBFFBBFRLL"));

    // vector of ids
    let mut vec: Vec<u64> = aoc::read_lines("input/day5.txt")
        .map(|line| parse(&line.unwrap()))
        .collect();

    // sort vector
    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
    vec.sort();

    // part1
    let max = vec.last().unwrap();
    println!("{}", max);

    // part2
    let ans = (1..vec.len()).find(|i| {
        vec[*i] - vec[i - 1] == 2
    });
    println!("{:?}", ans);
}

// borading pass to id
// e.g.
//  BFFFBBFRRR => 567
fn parse(s: &str) -> u64 {
    let (row, col) = s.chars().fold((0, 0), |(row, col), c| match c {
        'F' => (row << 1, col),
        'B' => ((row << 1) + 1, col),
        'L' => (row, col << 1),
        'R' => (row, (col << 1) + 1),
        _ => panic!(format!("unexpected char  {}", c)),
    });
    (row << 3) + col
}
