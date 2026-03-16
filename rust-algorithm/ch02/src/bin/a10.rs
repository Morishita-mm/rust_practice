use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    }

    let mut p = vec![0; n + 1];
    let mut s = vec![0; n + 2];

    for i in 1..=n {
        p[i] = max(p[i - 1], a[i - 1]);

        let j = n - i + 1;
        s[j] = max(s[j + 1], a[j - 1]);
    }

    for (l, r) in lr {
        println!("{}", max(p[l - 1], s[r + 1]));
    }
}
