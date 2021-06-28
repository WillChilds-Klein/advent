use std::{env, fs, io};
use std::io::BufRead;
use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum OpType {
    Mask,
    Write,
}

#[derive(Debug)]
struct Op {
    addr: u64,
    val: u64,
    mask: String,
    op_type: OpType,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        match &s[0..3] {
            "mem" => {
                let (addr, val) = match Regex::new(r"mem\[([\d]+)\] = ([\d]+)").unwrap().captures(s) {
                    Some(c) => (
                        c[1].parse::<u64>().unwrap(),
                        c[2].parse::<u64>().unwrap(),
                    ),
                    None => return Err(format!("Bad line {}", s)),
                };
                Ok(Op { op_type: OpType::Write, addr, val, mask: String::from("") })
            },
            "mas" => {
                let mask = match Regex::new(r"mask = ([01X]+)").unwrap().captures(s) {
                    Some(c) => c[1].to_string(),
                    None => return Err(format!("Bad line {}", s)),
                };
                Ok(Op { op_type: OpType::Mask, addr: 0, val: 0, mask })
            },
            _ => Err(format!("Invalid line: {}", s)),
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
    let ops: Vec<Op> = reader.lines()
        .map(Result::unwrap)
        .map(|s| Op::from_str(s.as_str()))
        .map(Result::unwrap)
        .collect();

    let acc = match vers.as_str() {
        "01" => one(&ops),
        "02" => two(&ops),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", acc);
}

fn one(ops: &Vec<Op>) -> u64 {
    let mut memory: HashMap<u64,u64> = HashMap::new();
    let mut curr_mask = "";
    for op in ops {
        match op.op_type {
            OpType::Mask => curr_mask = &op.mask,
            OpType::Write => drop(
                memory.insert(op.addr, mask_val(op.val, &curr_mask))
            ),
        }
    }
    return memory.iter().map(|(_,v)| v).sum();
}

fn mask_val(val: u64, mask: &str) -> u64 {
    let mut new_val = val;
    for ii in 0..mask.len() {
        match &mask[ii..ii+1] {
            "0" => new_val &= !(1 << (mask.len()-ii-1)),
            "1" => new_val |= 1 << (mask.len()-ii-1),
            "X" => continue,
            _ => panic!("Invalid mask character at addr {}: {}", ii, mask),
        }
    }
    return new_val;
}

fn two(ops: &Vec<Op>) -> u64 {
    let mut memory: HashMap<u64,u64> = HashMap::new();
    let mut curr_mask = "";
    for op in ops {
        match op.op_type {
            OpType::Mask => curr_mask = &op.mask,
            OpType::Write => {
                for addr in mask_addr(op.addr, &curr_mask) {
                    memory.insert(addr, op.val);
                }
            },
        }
    }
    return memory.iter().map(|(_,v)| v).sum();
}

fn mask_addr(addr: u64, mask: &str) -> Vec<u64> {
    let mut addrs = Vec::new();
    addrs.push(addr);
    for ii in 0..mask.len() {
        match &mask[ii..ii+1] {
            "0" => (),
            "1" => addrs.iter_mut().for_each(|a| *a |= 1 << (mask.len()-ii-1)),
            "X" => {
                addrs.iter_mut().for_each(|a| *a |= 1 << (mask.len()-ii-1));
                addrs.append(
                    &mut addrs.iter().map(|a| a & !(1 << (mask.len()-ii-1))).collect()
                );
            },
            _ => panic!("Invalid mask character at addr {}: {}", ii, mask),
        }
    }
    return addrs;
}
