use std::collections::HashMap;
use std::io::{self, BufRead};

struct Node {
    children: HashMap<String, Box<Node>>,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut lines = lock.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut root = Node::new();
    let mut count: usize = 0;

    for _ in 0..n {
        let s: String = lines.next().unwrap().unwrap();
        let segments: Vec<&str> = s.split('/').collect();
        let mut current: &mut Node = &mut root;

        for &seg in &segments[1..] {
            if seg.is_empty() {
                continue;
            }
            if !current.children.contains_key(seg) {
                count += 1;
                current
                    .children
                    .insert(seg.to_string(), Box::new(Node::new()));
            }
            let child = current.children.get_mut(seg).unwrap();
            current = child.as_mut();
        }
    }

    println!("{}", count);
}
