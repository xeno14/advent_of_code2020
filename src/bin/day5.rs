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
    for i in 1..vec.len() {
        if vec[i] - vec[i - 1] == 2 {
            println!("{}", vec[i] - 1);
            break;
        }
    }
}

// borading pass to id
// e.g.
//  BFFFBBFRRR => 567
fn parse(s: &str) -> u64 {
    let mut row = 0u64;
    let mut col = 0u64;

    for c in s.chars() {
        match c {
            'F' => row = row << 1,
            'B' => row = (row << 1) + 1,
            'L' => col = col << 1,
            'R' => col = (col << 1) + 1,
            _ => (),
        }
    }
    let id = (row << 3) + col;
    id
}
