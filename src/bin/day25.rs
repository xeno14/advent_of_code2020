fn next_step(n: u64, subject: &u64) -> u64 {
    let n = n * subject; 
    let n = n % 20201227;
    n
}

fn find_loop(pub1: u64, pub2: u64) -> (u64, u64) {
    let subject = 7;
    let mut n = 1;
    let mut nloop = 1;

    loop {
        n = next_step(n, &subject);
        if n == pub1 {
           return (pub2, nloop);
        }
        if n == pub2 {
            return (pub1, nloop);
        }
        nloop += 1;
    }
}

fn main() {
    let pub1 = 3248366;
    let pub2 = 4738476;

    let (subject, nloop) = find_loop(pub1, pub2);

    let mut n = 1;
    for _ in 0..nloop {
        n = next_step(n, &subject);
    }
    println!("{}", n);
}