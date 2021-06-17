use std::{env, fs, io};
use std::io::BufRead;
use std::cmp;

#[derive(Debug)]
enum Direction {
    Back,
    Front,
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Direction, Self::Err> {
        match s {
            "B" => Ok(Direction::Back),
            "F" => Ok(Direction::Front),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Unsupported Direction enum value!".to_string())
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
    let seats: Vec<Vec<Direction>> = reader.lines()
        .into_iter()
        .map(|line|
            line.unwrap()
                .split("")
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(str::parse::<Direction>)
                .map(Result::unwrap)
                .collect()
        )
        .collect();

    match vers.as_str() {
        "01" => one(seats),
        "02" => two(seats),
        _ => panic!("Unsupported vers: {}", vers)
    }
}

fn one(seats: Vec<Vec<Direction>>) {
    let mut max_seat_id = -1;
    for seat in seats {
        let mut row_lo = 0;
        let mut row_hi = 127;
        let mut col_lo = 0;
        let mut col_hi = 7;
        for direction in seat {
            match direction {
                Direction::Back => row_lo += (row_hi-row_lo)/2+1,
                Direction::Front => row_hi -= (row_hi-row_lo)/2+1,
                Direction::Left => col_hi -= (col_hi-col_lo)/2+1,
                Direction::Right => col_lo += (col_hi-col_lo)/2+1,
            };
        }
        let row = cmp::min(row_lo, row_hi);
        let col = cmp::min(col_lo, col_hi);
        let seat_id = 8*row + col;
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }
    println!("{}", max_seat_id);
}

fn two(seats: Vec<Vec<Direction>>) {
    const N_ROWS: usize = 128;
    const N_COLS: usize = 8;
    let mut map = [false; N_COLS*N_ROWS];
    for seat in seats {
        let mut row_lo = 0;
        let mut row_hi = N_ROWS-1;
        let mut col_lo = 0;
        let mut col_hi = N_COLS-1;
        for direction in seat {
            match direction {
                Direction::Back => row_lo += (row_hi-row_lo)/2+1,
                Direction::Front => row_hi -= (row_hi-row_lo)/2+1,
                Direction::Left => col_hi -= (col_hi-col_lo)/2+1,
                Direction::Right => col_lo += (col_hi-col_lo)/2+1,
            };
        }
        let row = cmp::min(row_lo, row_hi);
        let col = cmp::min(col_lo, col_hi);
        map[8*row+col] = true;
    }

    let mut seat_id: i32 = -1;
    for ii in 1..map.len()-2 {
        if map[ii-1] && !map[ii] && map[ii+1] {
            seat_id = ii as i32;
            break;
        }
    }
    println!("{}", seat_id);
}