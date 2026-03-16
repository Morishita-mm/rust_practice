use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        days: [(usize, usize); n],
    }

    let mut diff = vec![0; d + 2];

    for (l, r) in days {
        diff[l] += 1;
        diff[r + 1] -= 1;
    }

    let mut cur = 0;
    for i in 1..=d {
        cur += diff[i];
        println!("{}", cur);
    }
}
