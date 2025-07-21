use std::{
    env, fs,
    io::{BufRead, stdin},
};

fn solve(input: Vec<String>) -> usize {
    let first_line = input.first().unwrap();

    let mut length = first_line.len();
    if length == 11 {
        return 0;
    }

    let mut wagons = 0;

    while length > 0 {
        length -= 9;

        if length > 0 {
            wagons += 1;
            length -= 1; // Account for the space between wagons
        }
    }

    wagons
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
