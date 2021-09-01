use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::Enumerate;

#[derive(Debug, Clone)]
enum Token {
    Add,
    Mult,
    OpenParen,
    CloseParen,
    Val(u32),
}

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let lines: Vec<Vec<Token>> = reader.lines().into_iter()
        .map(Result::unwrap)
        .map(parse_line)
        .collect();
    let res = match vers.as_str() {
        "01" => one(&lines),
        _ => panic!("Unsupported vers: {}", vers),
    };
    println!("{}", res);
}

fn parse_line(line: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for c in line.chars() {
        let t: Option<Token> = match c {
            '+' => Some(Token::Add),
            '*' => Some(Token::Mult),
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            '0'..='9' => Some(Token::Val(c.to_digit(10).unwrap())), // NOTE: assumes single-digit numbers
            _ => None,
        };
        match t {
            Some(token) => tokens.push(token),
            _ => (),
        }
    }
    return tokens;
}

// fn process_tokens(tokens: &[Token]) -> u32 {
fn process_tokens(mut tokens: impl Iterator<Item = Token>) -> u32 {
    let mut op: fn(u32, u32) -> u32 = |x, y| y; // first time this is called, ignore first arg
    let mut ret = 0;
    for token in tokens {
        match token {
            Token::Add => op = |x, y| x + y,
            Token::Mult => op = |x, y| x * y,
            Token::Val(v) => ret = op(ret, v),
            // Token::OpenParen => ret = op(ret, process_tokens(&tokens[i+1..tokens.len()])),
            Token::OpenParen => ret = op(ret, process_tokens(&tokens)),
            Token::CloseParen => return ret,
        }
    }
    ret
}

fn one(tokens: &Vec<Vec<Token>>) -> u32 {
    tokens.iter()
        .map(Vec::to_iter)
        .map(process_tokens)
        .sum()
}