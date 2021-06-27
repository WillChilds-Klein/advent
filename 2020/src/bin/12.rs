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
    let actions: Vec<(char, i32)> = reader.lines()
        .map(Result::unwrap)
        .map(|l| (l[0..1].chars().nth(0).unwrap(), l[1..].parse::<i32>().ok().unwrap()))
        .collect();

    let acc = match vers.as_str() {
        "01" => one(&actions),
        "02" => two(&actions),
        _ => panic!("Unsupported vers: {}", vers)
    };
    println!("{}", acc);
}

fn one(actions: &Vec<(char, i32)>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;
    println!("{}", -270 % 360);
    for (a, v) in actions {
        match a {
            'N' => y += v,
            'S' => y -= v,
            'E' => x += v,
            'W' => x -= v,
            'R' => dir = (dir + v).rem_euclid(360), // % is "remainder", not "modulo".
            'L' => dir = (dir - v).rem_euclid(360), // https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
            'F' => match dir {
                0 => y += v,
                90 => x += v,
                180 => y -= v,
                270 => x -= v,
                _ => panic!("Invalid directional heading: {}", dir),
            }
            _ => panic!("Unrecognized action: {}", a),
        }
    }
    return x.abs() + y.abs();
}


fn two(actions: &Vec<(char, i32)>) -> i32 {
    let mut x_w = 10;
    let mut y_w = 1;
    let mut x = 0;
    let mut y = 0;
    // println!("{}\t{}\t{}\t{}\t{}\t{}", "a", "v", "x", "y", "x_w", "y_w");
    // for _ in 0..43 { print!("-"); }; print!("\n");
    for (a, v) in actions {
        // println!("{}\t{}\t{}\t{}\t{}\t{}", a, v, x, y, x_w, y_w);
        match a {
            'N' => y_w += v,
            'S' => y_w -= v,
            'E' => x_w += v,
            'W' => x_w -= v,
            'R' => rotate((-1*v).rem_euclid(360) as f32, x, y, &mut x_w, &mut y_w),
            'L' => rotate(v.rem_euclid(360) as f32, x, y, &mut x_w, &mut y_w),
            'F' => {
                let d_x = x_w - x;
                let d_y = y_w - y;
                x +=v * d_x;
                y +=v * d_y;
                x_w = x + d_x;
                y_w = y + d_y;
            }
            _ => panic!("Unrecognized action: {}", a),
        }
    }
    // println!("FINAL\t\t{}\t{}\t{}\t{}", x, y, x_w, y_w);
    return x.abs() + y.abs();
}

// https://en.wikipedia.org/wiki/Rotation_%28mathematics%29#Two_dimensions
fn rotate(degrees: f32, x: i32, y: i32, x_w: &mut i32, y_w: &mut i32) {
    let x_tmp = *x_w - x;   // save off waypoint x, subtract x to normalize to the origin
    let y_tmp = *y_w - y;   // save off waypoint y, subtract y to normalize to the origin
    *x_w = x_tmp * f32::cos(degrees.to_radians()) as i32 - y_tmp * f32::sin(degrees.to_radians()) as i32;
    *y_w = x_tmp * f32::sin(degrees.to_radians()) as i32 + y_tmp * f32::cos(degrees.to_radians()) as i32;
    *x_w += x;                    // add back x to transpose from the origin
    *y_w += y;                    // add back y to transpose from the origin
}