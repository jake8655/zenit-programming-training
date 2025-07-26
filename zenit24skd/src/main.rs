use std::{
    env, fs,
    io::{BufRead, stdin},
};

fn solve(input: Vec<String>) -> &'static str {
    let input_numbers = input
        .iter()
        .skip(1)
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<_>>>();

    let n = input_numbers.len();
    let mut diagonal = vec![None; 2 * n - 1];
    for i in 0..n {
        for j in 0..n {
            let index = (i as isize - j as isize + (n as isize - 1)) as usize;

            match diagonal[index] {
                None => {
                    diagonal[index] = Some(input_numbers[i][j]);
                }
                Some(v) => {
                    if v != input_numbers[i][j] {
                        return "kopa smetia";
                    }
                }
            }
        }
    }

    return "dokonale diagonalne";
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
