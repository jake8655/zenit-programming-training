use std::{
    env, fs,
    io::{BufRead, stdin},
};

const U: usize = 4;

fn generate_leftover_map() -> [usize; 5] {
    let mut best = [18446744073709551615; 5];
    best[0] = 0;

    for i in 0..=U {
        for j in 0..=U {
            let total_stops = 7 * i + 13 * j;
            let leftover_category = total_stops % 5;
            if leftover_category > 0 && best[leftover_category] > total_stops {
                best[leftover_category] = total_stops;
            }
        }
    }

    best
}

fn solve(input: Vec<String>) -> String {
    let trips = input
        .iter()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut trip_results = vec![];
    for trip in trips.iter() {
        let leftover = trip % 5; // 0, 1, 2, 3, 4
        let best_map = generate_leftover_map();
        if *trip >= best_map[leftover] {
            trip_results.push("ANO");
        } else {
            trip_results.push("NIE");
        }
    }

    trip_results.join("\n")
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
