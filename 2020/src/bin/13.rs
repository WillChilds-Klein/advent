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
    let mut lines = reader.lines();
    let start_time = lines.next().unwrap().unwrap().parse::<i64>().unwrap();
    let buses: Vec<i64> = lines.next().unwrap().unwrap()
        .split(",")
        .map(str::parse::<i64>)
        .map(|r| match r {
            Ok(b) => b,
            Err(_) => -1,
        })
        .collect();

    let acc = match vers.as_str() {
        "01" => one(start_time, &buses),
        "02" => two(&buses),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", acc);
}

fn one(start_time: i64, buses: &Vec<i64>) -> i64 {
    let mut min_wait = i64::MAX;
    let mut min_wait_bus = 0;
    for bus in buses {
        if bus < &0 {
            continue;
        }
        let wait = bus - (start_time % bus);
        if wait < min_wait {
            min_wait = wait;
            min_wait_bus = *bus;
        }
    }
    return min_wait * min_wait_bus;
}

// https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving
fn two(buses: &Vec<i64>) -> i64 {
    // .0 is the modulus m, .1 is congruence for the "candidate" modulo m
    let mut moduli: Vec<(i64,i64)> = buses.iter().enumerate()
        .filter(|(_, val)| **val > 0)
        .map(|(offset, m)| (*m as i64, (m-(offset as i64 % m)) % m))
        .collect();
    // sort in descending order to increase sieve efficiency
    moduli.sort_by_key(|(m, _)| *m);
    moduli.reverse();
    let mut step = 1;
    let mut candidate = 0;
    for (m, a) in moduli {
        while candidate % m != a {  // test if candidate mod m matches expected congruence
            candidate += step;
        }
        step *= m;                  // multiply step by n to reduce search space
    }
    return candidate;
}
