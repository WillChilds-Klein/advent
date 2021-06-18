use std::{env, fs, io};
use std::io::BufRead;

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let mut groups: Vec<[i32; 26]> = Vec::new();
    let mut group = [0; 26];
    let mut group_counts: Vec<i32> = Vec::new();
    let mut group_count = 0;
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        if line.is_empty() {
            groups.push(group);
            group = [0; 26];
            group_counts.push(group_count);
            group_count = 0;
        } else {
            for c in line.chars() {
                let ind = (c as usize) - 97;
                group[ind] += 1;
            }
            group_count += 1;
        }
    }
    groups.push(group);
    group_counts.push(group_count);

    match vers.as_str() {
        "01" => one(groups),
        "02" => two(groups, group_counts),
        _ => panic!("Unsupported vers: {}", vers)
    }
}

fn one(groups: Vec<[i32; 26]>) {
    let mut total = 0;
    for group in groups {
        for q in group.iter() {
            if q > &0 {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

fn two(groups: Vec<[i32; 26]>, group_counts: Vec<i32>) {
    assert_eq!(groups.len(), group_counts.len());
    let mut total = 0;
    for ii in 0..groups.len() {
        let group = groups[ii];
        for q in group.iter() {
            if *q == group_counts[ii] {
                total += 1;
            }
        }
    }
    println!("{}", total);
}
