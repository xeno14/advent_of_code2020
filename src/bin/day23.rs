use itertools::Itertools;

fn parse(s: &str) -> Vec<usize> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn main() {
    // let input_string = "389125467";
    let input_string = "219347865";
    let input: Vec<usize> = parse(input_string);

    // 'next' represents a circular linked list
    let mut next: Vec<usize> = vec![0; input.len() + 1];
    for (cur, nxt) in input.iter().tuple_windows() {
        next[*cur] = *nxt;
    }
    next[input[input.len() - 1]] = input[0]; // circulate

    // steps
    const MAX: usize = 9;
    let mut cur = input[0];
    for i in 1..101 {
        let mut pickup = vec![0; 3];
        pickup[0] = next[cur];
        pickup[1] = next[pickup[0]];
        pickup[2] = next[pickup[1]];
        let dst = find_dst(cur, &pickup, MAX);

        println!("-- move {} --", i);
        println!("cur: {}", cur);
        println!("next: {:?}", next);
        print_cups(cur, &next);
        println!("pick up: {:?}", pickup);
        println!("destination: {}", dst);
        println!("");

        let next_pickup = next[pickup[2]];
        let next_dst = next[dst];
        next[cur] = next_pickup;
        next[dst] = pickup[0];
        next[pickup[2]] = next_dst;
        cur = next[cur];
    }

    println!("{:?}", next);
    print_cups(1, &next);
    // print answer
}

fn find_dst(cur: usize, pickup: &Vec<usize>, max: usize) -> usize {
    let mut candidates: Vec<usize> = vec![0; 4];
    candidates[0] = if cur == 1 { max } else { cur - 1 };
    candidates[1] = if candidates[0] == 1 {
        max
    } else {
        candidates[0] - 1
    };
    candidates[2] = if candidates[1] == 1 {
        max
    } else {
        candidates[1] - 1
    };
    candidates[3] = if candidates[2] == 1 {
        max
    } else {
        candidates[2] - 1
    };

    for c in candidates {
        if !pickup.contains(&c) {
            return c;
        }
    }
    panic!();
}

fn print_cups(cur: usize, next: &Vec<usize>) {
    let mut c = cur;
    print!("{}", cur);
    for _ in 0..(next.len() - 2) {
        c = next[c];
        print!("{}", c);
    }
    println!("");
}
