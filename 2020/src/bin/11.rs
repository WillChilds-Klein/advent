use std::{env, fs, io, cmp};
use std::io::BufRead;

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let mut seats: Vec<Vec<char>> = Vec::new();
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        seats.push(line.chars().collect());
    }

    let acc = match vers.as_str() {
        "01" => one(&seats),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", acc);
}

fn one(seats: &Vec<Vec<char>>) -> u64 {
    print_seats(seats, "ORIGINAL");
    let mut prev_occupied = 0;
    let mut occupied = 1;
    let mut curr = seats.clone();
    let mut next = seats.clone();
    while occupied != prev_occupied {
        prev_occupied = occupied;
        occupied = iterate(&curr, &mut next);
        curr = next.clone();
    }
    print_seats(&next, "FINAL");
    return occupied;
}

fn iterate(seats: &Vec<Vec<char>>, next: &mut Vec<Vec<char>>) -> u64 {
    let mut total_occupied: u64 = 0;
    for ii in 0..seats.len() {
        for jj in 0..seats[0].len() {
            let adj_occupied = adjacent_occupied(seats, ii as isize, jj as isize);
            match seats[ii][jj] {
                'L' => {
                    if adj_occupied == 0 {
                        next[ii][jj] = '#';
                    }
                },
                '#' => {
                    if adj_occupied >= 4 {
                        next[ii][jj] = 'L';
                    }
                },
                _ => (),
            }
            match next[ii][jj] {
                '#' => {
                    total_occupied += 1;
                },
                _ => (),
            }
        }
    }
    // print_seats(next, "NEXT");
    return total_occupied;
}

fn adjacent_occupied(seats: &Vec<Vec<char>>, row_idx: isize, col_idx: isize) -> u64 {
    let mut occupied_count = 0;
    for ii in cmp::max(row_idx-1, 0)..cmp::min(row_idx+2, seats.len() as isize) {
        for jj in cmp::max(col_idx-1, 0)..cmp::min(col_idx+2, seats[0].len() as isize) {
            if ii == row_idx && jj == col_idx {
                continue;
            }
            match seats[ii as usize][jj as usize] {
                '#' => occupied_count += 1,
                _ => (),
            }
        }
    }
    return occupied_count;
}

fn print_seats(seats: &Vec<Vec<char>>, label: &str) {
    println!("{}", label);
    for (ii, row) in seats.iter().enumerate() {
        print!("| ");
        for c in row {
            print!("{}", c);
        }
        print!(" | ");
        for jj in 0..row.len() {
            print!("{}", adjacent_occupied(seats, ii as isize, jj as isize));
        }
        print!(" |");
        println!();
    }
    if !seats.is_empty() {
        for _ in 0..(seats[0].len()+3) {
            print!("-");
        }
        print!("+");
        for _ in 0..(seats[0].len()+2) {
            print!("-");
        }
        print!("+");
    }
    println!();
}