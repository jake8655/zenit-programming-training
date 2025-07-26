use std::{
    env, fs,
    io::{BufRead, stdin},
};

const PAIRS: [[usize; 2]; 7] = [[0, 0], [1, 1], [2, 2], [5, 5], [8, 8], [6, 9], [9, 6]];
const PAIRS_NON_ZERO: [[usize; 2]; 6] = [[1, 1], [2, 2], [5, 5], [8, 8], [6, 9], [9, 6]];

fn count_strobogrammatic_non_zero(k: usize) -> usize {
    if k == 0 {
        return 1;
    }

    if k == 1 {
        return PAIRS
            .iter()
            .filter(|pair| ![6, 9].contains(&pair[0]))
            .count();
    }

    return PAIRS_NON_ZERO.len() * count_strobogrammatic(k - 2);
}

fn count_strobogrammatic(k: usize) -> usize {
    if k == 0 {
        return 1;
    }

    if k == 1 {
        return PAIRS
            .iter()
            .filter(|pair| ![6, 9].contains(&pair[0]))
            .count();
    }

    return PAIRS.len() * count_strobogrammatic(k - 2);
}

fn solve(input: Vec<String>) -> i32 {
    let k = input[0].parse::<usize>().unwrap();

    count_strobogrammatic_non_zero(k) as i32
}

fn main() {
    let mut args = env::args().skip(1);
    let input = if let Some(arg) = args.next() {
        match arg.as_str() {
            "--file" => {
                let contents = fs::read_to_string("./input").unwrap_or_else(|e| {
                    eprintln!("Failed to read input file: {}", e);
                    std::process::exit(1);
                });
                let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
                lines
            }
            _ => stdin_lines(),
        }
    } else {
        stdin_lines()
    };

    let result = solve(input);
    println!("{}", result);
}

fn stdin_lines() -> Vec<String> {
    let sin = stdin();
    let lock = sin.lock();
    let lines = lock.lines().collect::<Result<_, _>>();
    match lines {
        Ok(lines) => lines,
        Err(_) => panic!("Failed to read lines from stdin"),
    }
}
