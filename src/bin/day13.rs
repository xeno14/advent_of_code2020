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
}
