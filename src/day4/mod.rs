use std::fs::read_to_string;
use std::panic::panic_any;

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
pub fn solve1() {
    let result = read_to_string("src/day4/input.txt");
    match result {
        Ok(res) => {
            let mut result: i64 = 0;

            let mut int_matrix: Vec<Vec<i8>> = Vec::new();

            for line in res.lines() {
                let v: Vec<i8> = line
                    .bytes() // iterate over ASCII bytes
                    .map(|b| i8::from(b == b'@'))
                    .collect();
                int_matrix.push(v);
            }

            let dirs: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

            let rows = int_matrix.len();
            let cols = int_matrix[0].len();

            for r in 0..rows {
                for c in 0..cols {
                    if int_matrix[r][c] != 1 {
                        continue;
                    }

                    let mut sum = 0;

                    for (dr, dc) in dirs {
                        let nr: i32 = i32::try_from(r).unwrap() + dr;
                        let nc: i32 = i32::try_from(c).unwrap() + dc;

                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            let i = int_matrix[nr as usize][nc as usize];
                            sum += i;
                        }
                    }

                    if sum < 4 {
                        result += 1;
                    }
                }
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
pub fn solve2() {
    let result = read_to_string("src/day4/input.txt");
    match result {
        Ok(res) => {
            let mut result: i64 = 0;

            let mut int_matrix: Vec<Vec<i8>> = Vec::new();

            for line in res.lines() {
                let v: Vec<i8> = line
                    .bytes() // iterate over ASCII bytes
                    .map(|b| i8::from(b == b'@'))
                    .collect();
                int_matrix.push(v);
            }

            let dirs: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

            let rows = int_matrix.len();
            let cols = int_matrix[0].len();
            let mut flag = true;

            while flag {
                flag = false;
                let mut int_matrix2 = int_matrix.clone();

                for r in 0..rows {
                    for c in 0..cols {
                        if int_matrix[r][c] != 1 {
                            continue;
                        }

                        let mut sum = 0;

                        for (dr, dc) in dirs {
                            let nr: i32 = i32::try_from(r).unwrap() + dr;
                            let nc: i32 = i32::try_from(c).unwrap() + dc;

                            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                                let i = int_matrix[nr as usize][nc as usize];
                                sum += i;
                            }
                        }

                        if sum < 4 {
                            int_matrix2[r][c] = 0;
                            result += 1;
                            flag = true;
                        }
                    }
                }

                int_matrix = int_matrix2;
            }

            println!("{result}");
        }
        Err(_) => panic_any("file not found"),
    }
}
