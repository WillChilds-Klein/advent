use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use regex::Regex;

trait Passport {
    fn new() -> Self;
    fn parse(pairs: Vec<(String,String)>) -> Result<Self,String> where Self: Sized;
    fn is_valid(&self) -> bool;
}

#[derive(Debug)]
struct PassportV1 {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport for PassportV1 {
    fn new() -> Self {
        PassportV1 {
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

    fn parse(pairs: Vec<(String, String)>) -> Result<Self, String> {
        let mut passport = PassportV1::new();
        for pair in pairs {
            let (k, v) = pair;
            match k.as_ref() {
                "byr" => passport.byr = Some(v.to_string()),
                "iyr" => passport.iyr = Some(v.to_string()),
                "eyr" => passport.eyr = Some(v.to_string()),
                "hgt" => passport.hgt = Some(v.to_string()),
                "hcl" => passport.hcl = Some(v.to_string()),
                "ecl" => passport.ecl = Some(v.to_string()),
                "pid" => passport.pid = Some(v.to_string()),
                "cid" => passport.cid = Some(v.to_string()),
                _ => () // do nothing on unrecognized key
            }
        }
        return Ok(passport);
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
            // ignore cid
    }
}

#[derive(Debug)]
enum Ecl {
    Default,    // used as placeholder for default, not valid data value
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth
}

impl std::str::FromStr for Ecl {
    type Err = String;
    fn from_str(s: &str) -> Result<Ecl, Self::Err> {
        match s {
            "amb" => Ok(Ecl::Amb),
            "blu" => Ok(Ecl::Blu),
            "brn" => Ok(Ecl::Brn),
            "gry" => Ok(Ecl::Gry),
            "grn" => Ok(Ecl::Grn),
            "hzl" => Ok(Ecl::Hzl),
            "oth" => Ok(Ecl::Oth),
            _ => Err("Unsupported Ecl enum value!".to_string())
        }
    }
}

#[derive(Debug)]
struct PassportV2 {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: String,
    hcl: String,
    ecl: Ecl,
    pid: String,
    cid: Option<i32>,
}

impl Passport for PassportV2 {
    fn new() -> Self {
        PassportV2 {
            byr: 0,
            iyr: 0,
            eyr: 0,
            hgt: String::from(""),
            hcl: String::from(""),
            ecl: Ecl::Default,
            pid: String::from(""),
            cid: None,
        }
    }

    fn parse(pairs: Vec<(String, String)>) -> Result<Self, String> {
        let mut passport = PassportV2::new();
        for pair in pairs {
            let (k, v) = pair;
            match k.as_ref() {
                "byr" => match v.parse::<i32>() {
                    Ok(i) => passport.byr = i,
                    Err(e) => return Err(e.to_string())
                }
                "iyr" => match v.parse::<i32>() {
                    Ok(i) => passport.iyr = i,
                    Err(e) => return Err(e.to_string())
                }
                "eyr" => match v.parse::<i32>() {
                    Ok(i) => passport.eyr = i,
                    Err(e) => return Err(e.to_string())
                }
                "hgt" => passport.hgt = String::from(v),
                "hcl" => passport.hcl = String::from(v),
                "ecl" => match v.parse::<Ecl>() {
                    Ok(ecl) => passport.ecl = ecl,
                    Err(e) => return Err(e)
                }
                "pid" => passport.pid = String::from(v),
                "cid" => match v.parse::<i32>() {
                    Ok(i) => passport.cid = Some(i),
                    Err(e) => return Err(e.to_string())
                }
                _ => return Err("Unsupported key!".to_string())
            }
        }
        return Ok(passport);
    }

    fn is_valid(&self) -> bool {
        let byr_valid = self.byr >= 1920 && self.byr <= 2002;
        let iyr_valid = self.iyr >= 2010 && self.iyr <= 2020;
        let eyr_valid = self.eyr >= 2020 && self.eyr <= 2030;
        let (hgt_value, hgt_unit) = match Regex::new(r"^([\d]+)([(in|cm)]+)$").unwrap().captures(&self.hgt) {
            Some(captures) => (
                captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(2).unwrap().as_str()
            ),
            None => return false
        };
        let hgt_valid = match hgt_unit {
            "cm" => hgt_value >= 150 && hgt_value <= 193,
            "in" => hgt_value >= 59 && hgt_value <= 76,
            _ => false
        };
        let hcl_valid = Regex::new(r"^#[a-f0-9]{6}$").unwrap().is_match(&self.hcl);
        let ecl_valid = match self.ecl {
            Ecl::Default => false,
            _ => true
        };
        let pid_valid = Regex::new(r"^[0-9]{9}$").unwrap().is_match(&self.pid);
        return byr_valid
            && iyr_valid
            && eyr_valid
            && hgt_valid
            && hcl_valid
            && ecl_valid
            && pid_valid
            // ignore cid
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
    let mut cur_pairs: Vec<(String,String)> = Vec::new();
    let mut all_pairs: Vec<Vec<(String,String)>> = Vec::new();
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        if line.is_empty() {
            all_pairs.push(cur_pairs);
            cur_pairs = Vec::new();
        } else {
            let line_pairs: Vec<(String,String)> = line.split(" ")
                .map(|s| s.split(":").map(str::to_string).collect())
                .map(|p: Vec<String>| (p[0].clone(), p[1].clone()))
                .collect();
            cur_pairs.extend(line_pairs);
        }
    }
    if !cur_pairs.is_empty() {
        all_pairs.push(cur_pairs);
    }

    match vers.as_str() {
        "01" => process::<PassportV1>(all_pairs),
        "02" => process::<PassportV2>(all_pairs),
        _ => panic!("Unsupported vers: {}", vers)
    }
}

fn process<P: Passport>(pairs: Vec<Vec<(String,String)>>) {
    let passports: Vec<P> = pairs.iter()
        .map(|p| match P::parse(p.to_vec()) {
            Ok(v) => Some(v),
            Err(_) => None
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .filter(P::is_valid)
        .collect();
    println!("{}", passports.len());
}