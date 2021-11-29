use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
use std::str::FromStr;
use std::string::ParseError;

use regex::Regex;

#[derive(Debug)]
struct Rule {
    id: u32,
    str: Option<String>,
    clauses: Vec<Vec<u32>>,
}

impl Rule {
    pub fn matches(&self, message: &String, _rules: &HashMap<u32, Rule>, _idx: usize) -> bool {
        // TODO [childw]
        for _clause in &self.clauses {
            if !message.is_empty() {
                return true;
            }
        }
        return false;
    }
}

impl FromStr for Rule {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut str = None;
        let mut clauses = Vec::new();
        let id: u32 = match Regex::new(r"^([\d]+): (.+)$").unwrap().captures(s) {
            Some(captures) => {
                let rest = captures.get(2).unwrap().as_str();
                match Regex::new(r#""([a-zA-Z0-9])+""#).unwrap().captures(rest) {
                    Some(c) => {
                        str = Some(c.get(1).unwrap().as_str().to_string());
                    },
                    None => {
                        clauses = rest.split(" | ").map(
                            |ids|ids
                                .split(" ")
                                .map(|id| id.parse::<u32>().unwrap())
                                .collect()
                        ).collect();
                    },
                }
                captures.get(1).unwrap().as_str().parse::<u32>().unwrap()
            },
            None => panic!("TODO [childw] make this a recoverable error")
        };
        Ok(Self{id, str, clauses})
    }
}

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>");
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let mut rules_vec = Vec::new();
    let mut messages: Vec<String> = Vec::new();
    let mut brk = false;
    let lines_iter = io::BufReader::new(infile).lines().into_iter();
    for wrapped_line in lines_iter {
        let line = wrapped_line.unwrap();
        if line.is_empty() {
            brk = true
        } else if !brk {
            rules_vec.push(Rule::from_str(&line));
        } else {
            messages.push(line.to_string());
        }
    }
    let rules: HashMap<u32,Rule> = HashMap::from_iter(
        rules_vec.into_iter().map(Result::unwrap).map(|r| (r.id, r))
    );

    let res = match vers.as_str() {
        "01" => one(&rules, &messages),
        _ => panic!("Unsupported vers: {}", vers),
    };
    println!("{}", res);
}

fn one(rules: &HashMap<u32,Rule>, messages: &Vec<String>) -> u32 {
    let mut valid_count = 0;
    for message in messages.into_iter() {
        if rules.get(&0).unwrap().matches(message, rules, 0) {
            valid_count += 1;
        }
    }
    valid_count
}