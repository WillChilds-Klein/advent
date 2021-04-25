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
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines() {
        input.push(line.unwrap().parse().unwrap());
    }

    match vers.as_str() {
        "01" => one(input),
        "02" => two(input),
        _ => panic!(format!("Unsupported vers: {}", vers))
    }
}

fn one(input: Vec<String>) {
    let mut valid: i32 = 0;
    for line in input {
        let parts: Vec<&str> = line.split(" ").collect();
        let range: &str = parts[0];
        let min = range.split("-").nth(0).unwrap().parse::<i32>().unwrap();
        let max = range.split("-").nth(1).unwrap().parse::<i32>().unwrap();
        let target = parts[1].chars().nth(0).unwrap();
        let password= parts[2];
        let target_count = password.matches(target).count() as i32;
        if target_count >= min && target_count <= max {
            valid = valid + 1;
        }
    }
    println!("{}", valid);
}

fn two(input: Vec<String>) {
    let mut valid: i32 = 0;
    for line in input {
        let parts: Vec<&str> = line.split(" ").collect();
        let range: &str = parts[0];
        let first = range.split("-").nth(0).unwrap().parse::<usize>().unwrap();
        let second = range.split("-").nth(1).unwrap().parse::<usize>().unwrap();
        let target = parts[1].chars().nth(0).unwrap();
        let password= parts[2];
        let target_count = password.chars()
            .enumerate()
            .filter(|&(i, c)| (i == first-1 || i == second-1) && c == target)
            .count();
        if target_count  == 1 {
            valid = valid + 1;
        }
    }
    println!("{}", valid);
}