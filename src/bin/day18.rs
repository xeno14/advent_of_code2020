/// Recursive Descent Parsing to parse formula

use aoc::read_lines;
use std::iter::Peekable;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Open,
    Close,
    Add,
    Mul,
    Num(u64),
}

impl Token {
    pub fn new(c: char) -> Token {
        match c {
            '(' => Token::Open,
            ')' => Token::Close,
            '+' => Token::Add,
            '*' => Token::Mul,
            c => {
                let val: u64 = c.to_digit(10).unwrap() as u64;
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

/// primary = num | "(" expr ")"
fn primary<'a, I>(tokens: &mut Peekable<I>) -> u64
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.next() {
        Some(token) => match token {
            Token::Num(val) => *val,
            Token::Open => {
                let num = expr(tokens);
                tokens.next();  // take Close
                num
            }
            _ => panic!()
        },
        None => panic!("expect token"),
    }
}

/// add = primary ("+" primary)*
fn add<'a, I>(tokens: &mut Peekable<I>) -> u64
where
    I: Iterator<Item = &'a Token>,
{
    let mut num = primary(tokens);
    while let Some(token) = tokens.peek() {
        let op = match token {
            Token::Add => {
                tokens.next();
                |x: u64, y: u64| x + y
            }
            _ => break,
        };
        num = op(num, primary(tokens));
    }
    num
}

/// expr = add ("*" add)*
fn expr<'a, I>(tokens: &mut Peekable<I>) -> u64
where
    I: Iterator<Item = &'a Token>,
{
    let mut num = add(tokens);
    while let Some(token) = tokens.peek() {
        let op = match token {
            Token::Mul => {
                tokens.next();
                |x, y| x * y
            }
            _ => break,
        };
        num = op(num, add(tokens))
    }
    num
}

fn eval(s: String) -> u64 {
    let tokens = Token::tokenize(s);
    expr(&mut tokens.iter().peekable())
}

fn main() {
    // // part1
    // assert_eq!(26, eval("2 * 3 + (4 * 5)".to_owned()));
    // assert_eq!(437, eval("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned()));
    // assert_eq!(
    //     12240,
    //     eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned())
    // );
    // assert_eq!(
    //     13632,
    //     eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned())
    // );
    // let ans: u64 = read_lines("input/day18.txt")
    //     .map(|line| line.unwrap())
    //     .map(eval)
    //     .sum();
    // println!("{}", ans);

    // part2
    assert_eq!(46, eval("2 * 3 + (4 * 5)".to_owned()));
    assert_eq!(1445, eval("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned()));
    assert_eq!(
        669060,
        eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned())
    );
    assert_eq!(
        23340,
        eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned())
    );
    let ans: u64 = read_lines("input/day18.txt")
        .map(|line| line.unwrap())
        .map(eval)
        .sum();
    println!("{}", ans);
}
