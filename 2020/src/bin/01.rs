use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let mut input: Vec<i32> = Vec::new();
    for line in reader.lines() {
        input.push(line.unwrap().parse().unwrap());
    }

    match vers.as_str() {
        "01" => one(input),
        "02" => two(input),
        _ => panic!(format!("Unsupported vers: {}", vers))
    }
}

fn one(input: Vec<i32>) {
    let target = 2020;
    let mut seen = HashSet::new();
    for expense in input.into_iter() {
        let counterpart = target - expense;
        if seen.contains(&counterpart) {
            println!("{}", expense * counterpart);
            return;
        }
        seen.insert(expense);
    }
}

fn two(input: Vec<i32>) {
    let target = 2020;
    let mut seen = HashMap::new();
    let mut ii = 0;
    for expense in input.iter() {
        let mut jj = ii + 1;
        while jj < input.len()-1 {
            let other = input[jj];
            let counterpart = expense + other;
            seen.insert(counterpart, (expense, other));
            jj += 1;
        }
        ii += 1;
    }

    for expense in input.iter() {
        let counterpart = target - expense;
        if seen.contains_key(&counterpart) {
            println!("{}", expense * seen.get(&counterpart).unwrap().0 * seen.get(&counterpart).unwrap().1);
            return;
        }
    }

    println!("NO MATCHES FOUND");
}