use aoc::read_lines;

fn main() {
    // part1
    let lines: Vec<Vec<char>> = read_lines("input/day3.txt")
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let ans = do_slope(&lines, 3, 1);
    println!("{}", ans);

    // part2
    let lines: Vec<Vec<char>> = read_lines("input/day3.txt")
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let ans: usize = slopes
        .into_iter()
        .map(|(dx, dy)| do_slope(&lines, dx, dy))
        .product();
    println!("{}", ans);
}

fn do_slope(lines: &Vec<Vec<char>>, dx: usize, dy: usize) -> usize {
    let height = lines.len();
    let width = lines[0].len();
    let count = (0..height)
        .filter(|i| {
            let x = dx * i % width;
            let y = dy * i;
            0 < y && y < height && '#' == lines[y][x]
        })
        .count();
    count
}
