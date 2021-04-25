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
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in input {
        let row: Vec<u8> = line.chars().map(|c| if c == '#' {1} else {0}).collect();
        map.push(row);
    }

    match vers.as_str() {
        "01" => one(map),
        "02" => two(map),
        _ => panic!(format!("Unsupported vers: {}", vers))
    }
}

fn one(map: Vec<Vec<u8>>) {
    let mut idx = 0;
    let mut count = 0;
    for row in map {
        if row[idx % row.len()] > 0 {
            count = count + 1;
        }
        idx = idx + 3;
    }
    println!("{}", count);
}

fn two(map: Vec<Vec<u8>>) {
    let slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let mut tree_counts: Vec<u64> = Vec::new();
    for [right, down] in slopes.iter() {
        let mut ii = 0;
        let mut jj = 0;
        let mut tree_count = 0;
        while ii < map.len() {
            let row = map.get(ii).unwrap();
            if row.get(jj).unwrap() > &0 {
                tree_count = tree_count + 1;
            }
            jj = (jj + right) % row.len();
            ii = ii + down;
        }
        tree_counts.push(tree_count);
    }
    println!("{}", tree_counts.iter().fold(1, |acc, v| acc * v));
}