use aoc::read_lines;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Open,
    Close,
    Add,
    Mul,
    Num(i64),
}

impl Token {
    pub fn new(c: char) -> Token {
        match c {
            '(' => Token::Open,
            ')' => Token::Close,
            '+' => Token::Add,
            '*' => Token::Mul,
            c => {
                let val: i64 = c.to_digit(10).unwrap() as i64;
                Token::Num(val)
            }
        }
    }

    pub fn tokenize(s: String) -> Vec<Token> {
        s.chars()
            .map(|c| match c {
                ' ' => None,
                c => Some(Token::new(c)),
            })
            .filter_map(|x| x)
            .collect()
    }
}

/// expr = primary (op primary)*
/// op = + | *
/// primary = num | "(" expr ")"
fn primary<'a, I>(tokens: &mut I) -> i64
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.next() {
        Some(token) => match token {
            Token::Num(val) => *val,
            Token::Open => expr(tokens),
            _ => panic!("expect num or open"),
        },
        None => panic!("expect token"),
    }
}

fn expr<'a, I>(tokens: &mut I) -> i64
where
    I: Iterator<Item = &'a Token>,
{
    let mut num = primary(tokens);
    while let Some(token) = tokens.next() {
        let op = match token {
            Token::Close => break,
            Token::Add => |x: i64, y: i64| x + y,
            Token::Mul => |x, y| x * y,
            _ => panic!(format!("Unexpected token {:?}", token)),
        };
        num = op(num, primary(tokens))
    }
    num
}

fn eval(s: String) -> i64 {
    let tokens = Token::tokenize(s);
    expr(&mut tokens.iter())
}

fn main() {
    // part1
    assert_eq!(26, eval("2 * 3 + (4 * 5)".to_owned()));
    assert_eq!(437, eval("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned()));
    assert_eq!(
        12240,
        eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned())
    );
    assert_eq!(
        13632,
        eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned())
    );
    let ans: i64 = read_lines("input/day18.txt")
        .map(|line| line.unwrap())
        .map(eval)
        .sum();
    println!("{}", ans);
}
