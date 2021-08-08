use std::{env, fs, io};
use std::io::BufRead;
use std::cmp;

fn main() {
    if env::args().count() < 3 {
        panic!("Usage: cargo run --bin <bin> <vers> <input_file>")
    }
    let vers = env::args().nth(1).unwrap();
    let path = env::args().nth(2).unwrap();
    let infile = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(infile);
    let flat: Vec<Vec<bool>> = reader.lines()
        .map(|line| line.unwrap()
            .split("")
            .filter(|c| *c == "#" || *c == ".")
            .map(|c| c == "#")
            .collect()
        )
        .collect();
    let initial_dimension = cmp::max(flat.len(), flat[0].len());
    let mut board = vec![vec![vec![false; initial_dimension]; initial_dimension]; 3];
    board[1] = flat.clone();

    let acc = match vers.as_str() {
        "01" => one(&board, 6),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", acc);
}

fn one(init_board: &Vec<Vec<Vec<bool>>>, iters: i32) -> i32 {
    let mut board = init_board.clone();
    print_board(&board);
    for _ in 0..iters {
        board = expand(&board);
        board = iterate(&board);
        print_board(&board);
    }
    let mut active_count = 0;
    for z in 0..board.len() {
        for y in 0..board[0].len() {
            for x in 0..board[0][0].len() {
                active_count += if board[z][y][x] {1} else {0};
            }
        }
    }
    active_count
}

fn count_active_neighbors(board: &Vec<Vec<Vec<bool>>>, z: i32, y: i32, x: i32) -> i32 {
    let mut count = 0;
    for ii in cmp::max(0,z-1)..cmp::min(z+2,board.len() as i32) {
        for jj in cmp::max(0,y-1)..cmp::min(y+2,board[0].len() as i32) {
            for kk in cmp::max(0,x-1)..cmp::min(x+2,board[0][0].len() as i32) {
                if  ii == z && jj == y && kk == x {
                    continue;
                }
                count += if board[ii as usize][jj as usize][kk as usize] {1} else {0};
            }
        }
    }
    count
}

fn iterate(board: &Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<bool>>> {
    let mut ret = board.clone();
    for z in 0..board.len() {
        for y in 0..board[0].len() {
            for x in 0..board[0][0].len() {
                let active_neighbors = count_active_neighbors(board, z as i32, y as i32, x as i32);
                ret[z][y][x] = if board[z][y][x] {
                    active_neighbors == 2 || active_neighbors == 3
                } else {
                    active_neighbors == 3
                };
            }
        }
    }
    ret
}

fn expand(board: &Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<bool>>> {
    let mut ret = board.clone();
    let [mut left, mut right, mut top, mut bottom, mut front, mut back] = [false; 6];
    for z in 0..board.len() {
        for y in 0..board[0].len() {
            for x in 0..board[0][0].len() {
                if board[z][y][x] {
                    if z == 0 && !front {
                        front = true;
                    }
                    if z == board.len()-1 && !back {
                        back = true;
                    }
                    if y == 0  && !top {
                        top = true;
                    }
                    if y == board[0].len()-1 && !bottom {
                        bottom = true;
                    }
                    if x == 0 && !left {
                        left = true;
                    }
                    if x == board.len()-1 && !right {
                        right = true;
                    }
                }
            }
        }
    }
    if front {
        let (h, w) = (ret[0].len(), ret[0][0].len());
        ret.insert(0, vec![vec![false; w]; h]);
    }
    if back {
        let (h, w) = (ret[0].len(), ret[0][0].len());
        ret.push(vec![vec![false; w]; h]);
    }
    if top {
        let (d, w) = (ret.len(), ret[0][0].len());
        for z in 0..d {
            ret[z].insert(0, vec![false; w]);

        }
    }
    if bottom {
        let (d, w) = (ret.len(), ret[0][0].len());
        for z in 0..d {
            ret[z].push(vec![false; w]);
        }
    }
    if left {
        let (d, h) = (ret.len(), ret[0].len());
        for z in 0..d {
            for y in 0..h {
                ret[z][y].insert(0, false);
            }
        }
    }
    if right {
        let (d, h) = (ret.len(), ret[0].len());
        for z in 0..d {
            for y in 0..h {
                ret[z][y].push(false);
            }
        }
    }
    ret
}

fn print_board(board: &Vec<Vec<Vec<bool>>>) {
    for y in 0..board[0].len() {
        print!("|");
        for z in 0..board.len() {
            for x in 0..board[0][0].len() {
                print!("{}", if board[z][y][x] {"█"} else {"X"});
            }
            print!("|");
        }
        println!();
    }
    println!("{:-<1$}", "", board[0][0].len()*board.len() + board.len()+1);
}