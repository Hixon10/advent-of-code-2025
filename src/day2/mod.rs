use std::fs::read_to_string;
use std::panic::panic_any;

pub fn solve1() {
    let result = read_to_string("src/day2/input.txt");
    match result {
        Ok(res) => {
            let mut result: i64 = 0;

            for line in res.split(',') {
                let (left, right) = line.split_once('-').unwrap();
                for i in left.parse::<i64>().unwrap()..=right.parse::<i64>().unwrap() {
                    let s = i.to_string();
                    if s.len() % 2 != 0 {
                        continue;
                    }
                    let (first, second) = s.split_at(s.len() / 2);
                    if first == second {
                        result += i;
                    }
                }
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}

fn is_repeated(s: &str, pat: &str) -> bool {
    if pat.is_empty() {
        return false;
    }
    let times = s.len() / pat.len();

    if pat.len() * times != s.len() {
        return false;
    }

    pat.repeat(times) == s
}

pub fn solve2() {
    let result = read_to_string("src/day2/input.txt");
    match result {
        Ok(res) => {
            let mut result: i64 = 0;

            for line in res.split(',') {
                let (left, right) = line.split_once('-').unwrap();
                for i in left.parse::<i64>().unwrap()..=right.parse::<i64>().unwrap() {
                    let s = i.to_string();

                    let n = s.len();

                    for j in 1..=n / 2 {
                        let prefix = &s[..j];
                        if is_repeated(&s, prefix) {
                            result += i;
                            break;
                        }
                    }
                }
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}
