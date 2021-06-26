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
        "02" => two(&seats),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", acc);
}

fn one(seats: &Vec<Vec<char>>) -> u64 {
    return process(seats, false);
}

fn two(seats: &Vec<Vec<char>>) -> u64 {
    return process(seats, true);
}

fn process(seats: &Vec<Vec<char>>, vis: bool) -> u64 {
    print_seats(seats, "ORIGINAL");
    let mut prev_occupied = 0;
    let mut occupied = 1;
    let mut curr = seats.clone();
    let mut next = seats.clone();
    while occupied != prev_occupied {
        prev_occupied = occupied;
        occupied = iterate(&curr, &mut next, vis);
        curr = next.clone();
    }
    print_seats(&next, "FINAL");
    return occupied;
}

fn iterate(seats: &Vec<Vec<char>>, next: &mut Vec<Vec<char>>, vis: bool) -> u64 {
    let mut total_occupied: u64 = 0;
    let occupied_threshold = if vis {5} else {4};
    for ii in 0..seats.len() {
        for jj in 0..seats[0].len() {
            let occupied = if vis {
                visible_occupied(seats, ii as isize, jj as isize)
            } else {
                adjacent_occupied(seats, ii as isize, jj as isize)
            };
            match seats[ii][jj] {
                'L' => {
                    if occupied == 0 {
                        next[ii][jj] = '#';
                    }
                },
                '#' => {
                    if occupied >= occupied_threshold {
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

fn visible_occupied(seats: &Vec<Vec<char>>, row_idx: isize, col_idx: isize) -> u64 {
    let mut count = 0;
    let transforms = [
        |x: isize| x-1, // decrement
        |x: isize| x,   // identity
        |x: isize| x+1, // increment
    ];
    for rt in transforms.iter() {
        for ct in transforms.iter() {
            if rt(0) == 0 && ct(0) == 0 { // skip if both rt and ct are identity, else infinite loop
                continue;
            } else if line_of_sight(seats, row_idx, col_idx, rt, ct) {
                count += 1;
            }
        }
    }
    return count;
}

fn line_of_sight(
    seats: &Vec<Vec<char>>, row: isize, col: isize, rt: &impl Fn(isize) -> isize, ct: &impl Fn(isize) -> isize
) -> bool {
    let mut ii = row;
    let mut jj = col;
    loop {
        ii = rt(ii);
        jj = ct(jj);
        if !(ii >= 0 && ii < seats.len() as isize && jj >= 0 && jj < seats[0].len() as isize) {
            return false;
        }
        if ii == row && jj == col { // don't consider origin coordinates
            continue;
        } else if seats[ii as usize][jj as usize] == '#' {  // occupied seat visible
            return true;
        } else if seats[ii as usize][jj as usize] == 'L' {  // empty seat visible
            return false;
        }
    }
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