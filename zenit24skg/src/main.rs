use std::cmp::max;
use std::io::{self, Read};

fn main() {
    // ---------- read input -------------------------------------------------
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<u64>().unwrap());

    let n = it.next().unwrap() as usize;
    let mut v = vec![0u64; n + 1]; // 1-based
    for i in 1..=n {
        v[i] = it.next().unwrap();
    }

    // ---------- quick impossibility: no triple at all ----------------------
    if n < 3 {
        println!("-1");
        return;
    }

    let k = (n as u64 - 1) / 2; // last index that can be chosen by Adam
    let k = k as usize;

    let mut need = vec![0u64; n + 1];
    let mut total: u128 = 0;
    let mut ok = true;

    // ---------- bottom-up traversal ---------------------------------------
    for i in (1..=n).rev() {
        if i > k {
            // cannot be chosen by Adam
            let parent = i / 2;
            if parent > k && v[i] > 0 {
                ok = false; // nobody can reach this carriage
                break;
            }
            need[i] = v[i];
        } else {
            // internal node, Adam can work here
            let left_need = if 2 * i <= n { need[2 * i] } else { 0 };
            let right_need = if 2 * i + 1 <= n { need[2 * i + 1] } else { 0 };

            let mut s_i = max(left_need, right_need);
            if i == 1 {
                s_i = max(s_i, v[1]); // the root has no parent
            }

            total += s_i as u128;

            let residual = v[i].saturating_sub(s_i);
            need[i] = residual;
        }
    }

    if !ok {
        println!("-1");
    } else {
        println!("{}", total);
    }
}
