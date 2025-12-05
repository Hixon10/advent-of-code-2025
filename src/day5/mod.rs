use std::cmp::{max, min};
use std::fs::read_to_string;
use std::panic::panic_any;

pub fn solve1() {
    let result = read_to_string("src/day5/input.txt");
    match result {
        Ok(res) => {
            let mut result: i64 = 0;
            let mut iter = res.lines();

            let mut pairs: Vec<(i64, i64)> = iter
                .by_ref()
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let parts: Vec<i64> = line.split('-').map(|n| n.parse::<i64>().unwrap()).collect();
                    (parts[0], parts[1])
                })
                .collect();

            let mut numbers: Vec<i64> = iter.filter(|l| !l.is_empty()).map(|l| l.parse::<i64>().unwrap()).collect();

            // Sort by the first element
            pairs.sort_by_key(|p| p.0);
            numbers.sort_unstable();

            let mut current_index = 0;

            for id in numbers {
                while current_index < pairs.len() {
                    let (left, right) = pairs[current_index];
                    if id < left {
                        break;
                    }
                    if id >= left && id <= right {
                        result += 1;
                        break;
                    }
                    current_index += 1;
                }
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}

pub fn solve2() {
    let result = read_to_string("src/day5/input.txt");
    match result {
        Ok(res) => {
            let mut result: i64 = 0;

            let mut pairs: Vec<(i64, i64)> = res
                .lines()
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let parts: Vec<i64> = line.split('-').map(|n| n.parse::<i64>().unwrap()).collect();
                    (parts[0], parts[1])
                })
                .collect();

            pairs.sort_by_key(|p| p.0);

            for i in 0..pairs.len() {
                // Iterates from 1 up to (but not including) 6
                if (i == pairs.len() - 1) || (pairs[i].1 < pairs[i + 1].0) {
                    result += pairs[i].1 - pairs[i].0 + 1;
                    continue;
                }

                pairs[i + 1].0 = min(pairs[i + 1].0, pairs[i].0);
                pairs[i + 1].1 = max(pairs[i + 1].1, pairs[i].1);
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}
