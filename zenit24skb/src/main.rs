use std::io::stdin;

fn main() {
    let train = stdin_i32();

    let opposite = if train % 2 == 0 { train + 1 } else { train - 1 };

    println!("{}", opposite);
}

fn stdin_i32() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input
        .trim()
        .parse()
        .expect("Failed to convert string to i32")
}
