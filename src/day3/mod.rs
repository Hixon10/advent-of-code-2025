use std::fs::read_to_string;
use std::panic::panic_any;

pub fn solve1() {
    let result = read_to_string("src/day3/input.txt");
    match result {
        Ok(res) => {
            let mut result: i64 = 0;

            for line in res.lines() {
                let mut left: u32 = 0;
                let mut right: u32 = 0;
                let mut index: usize = 0;

                for i in 0..line.len() - 1 {
                    let digit = u32::from(line.as_bytes()[i] - b'0');
                    if digit > left {
                        left = digit;
                        index = i;
                    }
                }

                for i in (index + 1)..line.len() {
                    let digit = u32::from(line.as_bytes()[i] - b'0');
                    if digit > right {
                        right = digit;
                    }
                }

                result += i64::from((10 * left) + right);
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}

fn find_max(s: &str, from: usize, to: usize) -> (u32, usize) {
    let mut current_max: u32 = 0;
    let mut index: usize = 0;

    for i in from..to {
        let digit = u32::from(s.as_bytes()[i] - b'0');
        if digit > current_max {
            current_max = digit;
            index = i;
        }
    }

    (current_max, index)
}

pub fn solve2() {
    let result = read_to_string("src/day3/input.txt");
    match result {
        Ok(res) => {
            let mut result: i64 = 0;

            for line in res.lines() {
                let mut index: usize = 0;

                for i in (0..12).rev() {
                    let (current_max, current_index) = find_max(line, index, line.len() - i);
                    index = current_index + 1;
                    let number = i64::pow(10, i as u32) * i64::from(current_max);
                    result += number;
                }
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}
