use aoc::read_lines;

fn main() {
    // let input = "input/day3-example.txt";
    let input = "input/day3-1.txt";
    let lines: Vec<String> = read_lines(input).map(|line| line.unwrap()).collect();
    let ans = part1(lines);
    println!("{}", ans);
}

fn part1(lines: Vec<String>) -> usize {
    lines
        .into_iter()
        .enumerate()
        .filter(|(y, line)| {
            if *y == 0usize {
                false
            } else {
                let x = 3 * y % line.len();
                '#' == line.chars().nth(x).unwrap()
            }
        })
        .count()
}