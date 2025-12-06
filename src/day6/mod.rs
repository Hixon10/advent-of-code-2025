use std::fs::read_to_string;
use std::panic::panic_any;

pub fn solve1() {
    let result = read_to_string("src/day6/input.txt");
    match result {
        Ok(res) => {
            let mut result: i64 = 0;
            let mut lines = res.lines().peekable();

            let mut numbers: Vec<Vec<i64>> = Vec::new();

            while let Some(line) = lines.next() {
                if lines.peek().is_none() {
                    for (index, op) in line.split_whitespace().enumerate() {
                        match op {
                            "+" => {
                                result += numbers.iter().filter_map(|row| row.get(index)).sum::<i64>();
                            }
                            "*" => {
                                result += numbers.iter().filter_map(|row| row.get(index)).product::<i64>();
                            }
                            _ => panic!("unknown operation: {op}"),
                        }
                    }

                    break; // this is the last line → skip it
                }

                let row = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();

                numbers.push(row);
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}

#[allow(clippy::needless_range_loop)]
fn process_submatrix(matrix: &[&[char]], op: char) -> i64 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result: i64 = i64::from(op == '*');

    for col in 0..cols {
        let mut digits = String::new();

        for row in 0..rows {
            let ch = matrix[row][col];
            if ch.is_ascii_digit() {
                digits.push(ch);
            }
        }

        if !digits.is_empty() {
            let value: i64 = digits.parse().unwrap();

            match op {
                '+' => {
                    result += value;
                }
                '*' => {
                    result *= value;
                }
                _ => panic!("unknown operation: {op}"),
            }
        }
    }

    result
}

pub fn solve2() {
    let result = read_to_string("src/day6/input.txt");
    match result {
        Ok(res) => {
            let mut result: i64 = 0;
            let mut lines = res.lines().peekable();

            let mut numbers: Vec<Vec<char>> = Vec::new();

            while let Some(line) = lines.next() {
                if lines.peek().is_none() {
                    let mut prev_idx_opt: Option<usize> = None;
                    let mut prev_op_opt: Option<char> = None;

                    for (index, op) in line.char_indices() {
                        match op {
                            '+' | '*' => {
                                if let (Some(prev_idx), Some(prev_op)) = (prev_idx_opt, prev_op_opt) {
                                    let view: Vec<&[char]> = numbers
                                        .iter()
                                        .map(|row| &row[prev_idx..index - 1]) // slice each row without copying
                                        .collect();

                                    result += process_submatrix(&view, prev_op);
                                }

                                prev_idx_opt = Some(index);
                                prev_op_opt = Some(op);
                            }
                            _ => {}
                        }
                    }

                    if let (Some(prev_idx), Some(prev_op)) = (prev_idx_opt, prev_op_opt) {
                        let view: Vec<&[char]> = numbers
                            .iter()
                            .map(|row| &row[prev_idx..]) // slice each row without copying
                            .collect();

                        result += process_submatrix(&view, prev_op);
                    }

                    break; // this is the last line → skip it
                }

                let row = line.chars().collect();

                numbers.push(row);
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}
