use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use core::cmp;

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<(u32,u32)>,
}

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let mut lines = reader.lines().into_iter();
    let mut fields: Vec<Field> = Vec::new();
    let mut line;
    loop {
        line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break
        }
        let name = line.split(": ").nth(0).unwrap().to_string();
        let ranges = line.split(": ").nth(1).unwrap().split(" or ")
            .map(|s|
                s.split("-")
                    .map(str::parse::<u32>)
                    .map(Result::unwrap)
                    .collect::<Vec<u32>>()
            )
            .map(|v| (v[0],v[1]))
            .collect();
        fields.push(Field { name, ranges });
    }

    let parse_ticket = |s: String| {
        s.split(",")
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect()
    };

    assert_eq!("your ticket:", lines.next().unwrap().unwrap());
    let ticket = parse_ticket(lines.next().unwrap().unwrap());
    assert_eq!("", lines.next().unwrap().unwrap());

    assert_eq!("nearby tickets:", lines.next().unwrap().unwrap());
    let mut nearby_tix: Vec<Vec<u32>> = Vec::new();
    loop {
        line = match lines.next() {
            Some(l) => l.unwrap(),
            None => break,
        };
        if line.is_empty() {
            break
        }
        nearby_tix.push(parse_ticket(line));
    }

    let res = match vers.as_str() {
        "01" => one(&fields, &ticket, &nearby_tix),
        "02" => two(&fields, &ticket, &nearby_tix),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", res);
}

fn one(fields: &Vec<Field>, _ticket: &Vec<u32>, nearby_tix: &Vec<Vec<u32>>) -> u32 {
    let valid_ranges: Vec<bool> = construct_valid_ranges(fields);
    let mut invalid_values: Vec<u32> = Vec::new();
    for t in nearby_tix {
        for v in t {
            if *v as usize > valid_ranges.len() || !valid_ranges[*v as usize] {
                invalid_values.push(*v);
            }
        }
    }
    return invalid_values.iter().sum();
}

fn two(fields: &Vec<Field>, ticket: &Vec<u32>, nearby_tix: &Vec<Vec<u32>>) -> u32 {
    let valid_ranges: Vec<bool> = construct_valid_ranges(fields);
    let valid_tix: Vec<&Vec<u32>> = nearby_tix.iter()
        .filter(|t| {
            for v in *t {
                if *v as usize > valid_ranges.len() || !valid_ranges[*v as usize] {
                    return false;
                }
            }
            return true;
        })
        .collect();
    // TODO: how to efficiently assign fields to range constraints?
    0
}

// construct a Vec<bool> representing valid regions. this saves us from doing n * m rules checks
fn construct_valid_ranges(fields: &Vec<Field>) -> Vec<bool> {
    let mut max = u32::MIN;
    for f in fields {
        for r in &f.ranges {
            let r_max = cmp::max(r.0, r.1);
            if r_max > max {
                max = r_max;
            }
        }
    }
    let mut valid: Vec<bool> = Vec::new();
    for _ in 0..(max+1) {
        valid.push(false);
    }
    for f in fields {
        for r in &f.ranges {
            for ii in r.0..((r.1)+1) {
                valid[ii as usize] = true;
            }
        }
    }
    return valid;
}
