use std::fs::read_to_string;
use std::panic::panic_any;

const fn wrap_add(a: i32, b: i32) -> i32 {
    (a + b).rem_euclid(100)
}

const fn wrap_sub(a: i32, b: i32) -> i32 {
    (a - b).rem_euclid(100)
}

pub fn solve1() {
    let result = read_to_string("src/day1/input.txt");
    match result {
        Ok(res) => {
            let mut result = 0;
            let mut current = 50;

            for line in res.lines() {
                let a = line[1..].parse().unwrap();

                if line.starts_with('L') {
                    current = wrap_sub(current, a);
                } else {
                    current = wrap_add(current, a);
                }

                if current == 0 {
                    result += 1;
                }
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}

pub fn solve2() {
    let result = read_to_string("src/day1/input.txt");
    match result {
        Ok(res) => {
            let mut result = 0;
            let mut current = 50;

            for line in res.lines() {
                let a: i32 = line[1..].parse().unwrap();
                let hits;

                if line.starts_with('L') {
                    let dist_to_zero = if current == 0 { 100 } else { current };

                    if a >= dist_to_zero {
                        let remaining = a - dist_to_zero;
                        hits = 1 + (remaining / 100);
                    } else {
                        hits = 0;
                    }

                    let diff = current - a;
                    current = ((diff % 100) + 100) % 100;
                } else {
                    let sum = current + a;
                    hits = sum / 100;
                    current = sum % 100;
                }

                result += hits;
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}
