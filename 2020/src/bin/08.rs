use std::{env, fs, io};
use std::io::BufRead;

use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
enum Op {
    Nop,
    Acc,
    Jmp
}

impl std::str::FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Op, Self::Err> {
        match s {
            "nop" => Ok(Op::Nop),
            "acc" => Ok(Op::Acc),
            "jmp" => Ok(Op::Jmp),
            _ => Err(format!("Unsupported Op enum value: {}", s))
        }
    }
}


fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let mut ops: Vec<(Op,i32)> = Vec::new();
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        match Regex::new(r"^([a-z]{3}) ([-+])([0-9]+)$").unwrap().captures(line.as_str()) {
            Some(c) => {
                let op = c.get(1).unwrap().as_str().parse::<Op>().unwrap();
                let mut val = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
                if c.get(2).unwrap().as_str() == "-" {
                    val *= -1;
                }
                ops.push((op, val));
            },
            None => panic!("Malformed line: {}", line)
        }
    }

    let acc = match vers.as_str() {
        "01" => one(ops),
        "02" => two(ops),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", acc);
}

fn one(ops: Vec<(Op,i32)>) -> i32{
    return compute(&ops).1;
}

fn two(ops: Vec<(Op,i32)>) -> i32 {
    let mut ops_copy = ops.to_vec();
    let toggle_op = |o: &Op| if *o == Op::Nop { Op::Jmp } else { Op::Nop };
    for ii in 0..ops.len() {
        if ops_copy[ii].0 != Op::Acc {
            ops_copy[ii] = (toggle_op(&ops_copy[ii].0), ops_copy[ii].1);
            let (term, acc) = compute(&ops_copy);
            if term {
                return acc;
            }
            ops_copy[ii] = (toggle_op(&ops_copy[ii].0), ops_copy[ii].1);
        }
    }
    panic!("No alterations result in halt!");
}

fn compute(ops: &Vec<(Op,i32)>) -> (bool, i32) {
    let mut seen = vec![false; ops.len()];
    let mut acc = 0;
    let mut ptr: i32 = 0;
    while ptr < ops.len() as i32 {
        if seen[ptr as usize] {
            return (false, acc);
        }
        seen[ptr as usize] = true;
        let (op, val) = &ops[ptr as usize];
        match op {
            Op::Jmp => ptr += *val,
            _ => ptr += 1,
        }
        match op {
            Op::Acc => acc += *val,
            _ => (),
        }
    }
    return (true, acc);
}