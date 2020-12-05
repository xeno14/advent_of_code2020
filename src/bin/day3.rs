use aoc::read_lines;

fn main() {
    // let input = "input/day3-example.txt";
    let input = "input/day3-1.txt";
    let lines: Vec<String> = read_lines(input).map(|line| line.unwrap()).collect();

    let mut ans: usize = 0;
    for (y, line) in lines.into_iter().enumerate() {
        if y == 0 { continue }
        let x = 3 * y % line.len();
        let c = line.chars().nth(x).unwrap();
        if c == '#' { ans += 1; continue }
    }
    println!("{}", ans);
}
