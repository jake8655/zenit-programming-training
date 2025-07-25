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

    let first_row = &input_numbers[0];

    for i in 0..first_row.len() - 1 {
        let current = first_row[i];
        let mut row = 1;
        for j in i + 1..input_numbers.len() {
            if input_numbers[row][j] != current {
                return "kopa smetia";
            }
            row += 1;
        }
    }

    for i in 1..input_numbers.len() - 1 {
        let current = input_numbers[i][0];
        let mut row = 1;
        for j in i + 1..input_numbers.len() {
            if input_numbers[j][row] != current {
                return "kopa smetia";
            }
            row += 1;
        }
    }

    "dokonale diagonalne"
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
