use std::{env, fs, io};
use std::collections::HashSet;
use std::io::BufRead;

const WINDOW: i64 = 25;

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let mut nums: Vec<i64> = Vec::new();
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        nums.push(line.parse().unwrap());
    }

    let acc = match vers.as_str() {
        "01" => one(&nums, WINDOW),
        "02" => two(&nums, WINDOW),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", acc);
}

fn one(nums: &Vec<i64>, preamble_len: i64) -> i64 {
    let mut preamble: Vec<i64> = Vec::new();
    for ii in 0..nums.len() {
        let num = nums[ii]   ;
        if ii < preamble_len as usize {
            preamble.insert(0, num);
            continue;
        }
        let mut found_pair = false;
        let mut seen: HashSet<i64> = HashSet::new();
        for p in &preamble {
            if seen.contains(p) {
                found_pair = true;
                break;
            }
            seen.insert(num-*p);
        }
        if !found_pair {
            return num;
        }
        preamble.pop();
        preamble.insert(0, num);
    }
    panic!("All numbers have additive components!");
}

fn two(nums: &Vec<i64>, preamble_len: i64) -> i64 {
    let target = one(nums, preamble_len);
    for ii in 0..nums.len() {
        let mut candidates: Vec<i64> = Vec::new();
        let mut candidate_sum = 0;
        let mut jj = ii;
        while candidate_sum < target {
            let candidate = nums[jj];
            jj += 1;
            candidates.push(candidate);
            candidate_sum += candidate;
        }
        if candidate_sum == target {
            let mut min = i64::MAX;
            let mut max = i64::MIN;
            for candidate in candidates {
                if candidate < min {
                    min = candidate;
                }
                if candidate > max {
                    max = candidate;
                }
            }
            return min + max;
        }
    }
    panic!("All numbers have additive components!");
}
