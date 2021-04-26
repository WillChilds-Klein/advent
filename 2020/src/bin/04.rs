use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Default for Passport {
    fn default() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
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
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport = Passport::default();
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        if line.is_empty() {
            passports.push(passport);
            passport = Passport::default();
        } else {
            let pairs: Vec<Vec<String>> = line.split(" ")
                .map(|s| s.split(":").map(str::to_string).collect())
                .collect();
            for pair in pairs {
                let k = pair.get(0).unwrap();
                let v = pair.get(1).unwrap();
                match k.as_ref() {
                    "byr" => passport.byr = Some(v.to_string()),
                    "iyr" => passport.iyr = Some(v.to_string()),
                    "eyr" => passport.eyr = Some(v.to_string()),
                    "hgt" => passport.hgt = Some(v.to_string()),
                    "hcl" => passport.hcl = Some(v.to_string()),
                    "ecl" => passport.ecl = Some(v.to_string()),
                    "pid" => passport.pid = Some(v.to_string()),
                    "cid" => passport.cid = Some(v.to_string()),
                    _ => panic!("unsupported key")
                }
            }
        }
    }
    passports.push(passport);

    match vers.as_str() {
        "01" => one(passports),
        _ => panic!(format!("Unsupported vers: {}", vers))
    }
}

fn one(passports: Vec<Passport>) {
    let valid_count = passports.iter()
        .filter(|p| {
            p.byr != None
                && p.iyr != None
                && p.eyr != None
                && p.hgt != None
                && p.hcl != None
                && p.ecl != None
                && p.pid != None
        })
        .count();
    println!("{}", valid_count);
}