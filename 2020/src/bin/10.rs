use std::{env, fs, io};
use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let mut nums: Vec<u64> = Vec::new();
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        nums.push(line.parse().unwrap());
    }

    nums.sort();
    nums.insert(0, 0);
    nums.push(nums.last().unwrap()+3);

    let acc = match vers.as_str() {
        "01" => one(&nums),
        "02" => two(&nums, &mut HashMap::new(), 0),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", acc);
}

fn one(nums: &Vec<u64>) -> u64 {
    let mut sums = [0; 4];
    for ii in 0..nums.len()-1 {
        let diff: usize = (nums[ii+1] - nums[ii]) as usize;
        sums[diff] += 1;
    }
    return sums[1] * sums[3];
}

fn two(nums: &Vec<u64>, cache: &mut HashMap<usize,u64>, idx: usize) -> u64 {
    if cache.contains_key(&idx) {
        return *cache.get(&idx).unwrap();
    }
    if idx == nums.len()-1 {
        return 1;
    }
    let mut sum = 0;
    for ii in idx+1..idx+4 {
        if ii < nums.len() && nums[ii] - nums[idx] < 4 {
            sum += two(nums, cache, ii);
        }
    }
    cache.insert(idx, sum);
    return sum;
}