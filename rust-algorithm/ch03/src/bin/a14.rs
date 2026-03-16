use proconio::input;
use std::{collections::HashSet, process::exit};

fn main() {
    input! {
        n: usize,
        k: i32,
        mut a: [i32;n],
        mut b: [i32;n],
        mut c: [i32;n],
        mut d: [i32;n],
    }
    let mut p = Vec::with_capacity(n * n);
    for &val_a in &a {
        for &val_b in &b {
            p.push(val_a + val_b);
        }
    }
    p.sort();

    for &val_c in &c {
        for &val_d in &d {
            let target = k - (val_c + val_d);

            if p.binary_search(&target).is_ok() {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
