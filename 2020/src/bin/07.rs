use std::{env, fs, io};
use std::collections::HashMap;
use std::io::BufRead;

use regex::Regex;

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let mut rules: HashMap<String,Vec<(String,i32)>> = HashMap::new();
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        let (lead, contents_string) = match Regex::new(r"^(.+) bags contain (.+)$").unwrap().captures(&line) {
            Some(c) => (
                c.get(1).unwrap().as_str().to_string(),
                c.get(2).unwrap().as_str().to_string(),
            ),
            None => panic!("no lead match"),
        };
        let contents = contents_string.split(", ")
            .into_iter()
            .map(|p| {
                let re = Regex::new(r"^(\d) (.+) bag").unwrap();
                match p {
                    "no other bags." => None,
                    p if re.is_match(p) => Some((
                        re.captures(p).unwrap().get(2).unwrap().as_str().to_string(),
                        re.captures(p).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    )),
                    _ => panic!("contents of unexpected format: {}", p),
                }
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        rules.insert(lead, contents);
    }

    match vers.as_str() {
        "01" => one(rules),
        "02" => two(rules),
        _ => panic!("Unsupported vers: {}", vers)
    }
}

fn one(rules: HashMap<String,Vec<(String,i32)>>) {
    let mut count = 0;
    const TARGET: &str = "shiny gold";
    for (lead, _) in &rules {
        let mut work: Vec<&String> = Vec::new();    // BFS work queue
        if lead == TARGET {                         // don't search TARGET entry
            continue;
        }
        work.push(lead);
        while !work.is_empty() {
            let curr = work.pop().unwrap();
            if curr.as_str() == TARGET {
                count += 1;
                work.clear();                       // reset work queue, go to next rule entry
                break;
            }
            match rules.get(curr) {
                Some(new_work) => new_work.iter().for_each(|(c,_)| work.push(&c)),
                None => panic!("No rule for {}!", curr),
            }
        }
    }
    println!("{}", count);
}

fn two(rules: HashMap<String,Vec<(String,i32)>>) {
    let mut bag_count = 0;
    const TARGET: &str = "shiny gold";
    let mut work: Vec<&str> = Vec::new();
    work.push(TARGET);
    while !work.is_empty() {
         let curr = work.pop().unwrap();
        match rules.get(curr) {
            Some(new_work) => {
                for (color,count) in new_work {
                    bag_count += count;
                    for _ in 0..*count {
                        work.push(color);
                    }
                }
            }
            None => panic!("No rule for {}!", curr),
        }
    }
    println!("{}", bag_count);
}
