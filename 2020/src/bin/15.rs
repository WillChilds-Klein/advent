use std::{env, fs, io};
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let mut lines = reader.lines();
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split(",")
        .map(str::parse::<i32>)
        .map(|r| match r {
            Ok(b) => b,
            Err(_) => -1,
        })
        .collect();

    let acc = match vers.as_str() {
        "01" => one(&nums, 2020),
        "02" => one(&nums, 30_000_000),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", acc);
}

fn one(starting_nums: &Vec<i32>, limit: usize) -> i32 {
    let mut seen: HashMap<i32,i32> = HashMap::new();
    for (ii, n) in starting_nums.iter().enumerate() {
        seen.insert(*n, ii as i32);
    }
    let mut prev = *starting_nums.last().unwrap();
    let mut curr = 0;
    for ii in starting_nums.len()..limit {
        let prev_ind = ii as i32 - 1;
        curr = match seen.get(&prev) {
            Some(p) => prev_ind - p,
            None => 0,
        };
        seen.insert(prev, prev_ind);
        prev = curr;
    }
    return curr;
}
