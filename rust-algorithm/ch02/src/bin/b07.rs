use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        times: [(usize, usize); n],
    }
    let mut diff = vec![0; t + 1];

    for (l, r) in times {
        diff[l] += 1;
        diff[r] -= 1;
    }

    let mut cur = 0;
    for i in 0..=t {
        cur += diff[i];
        println!("{cur}");
    }
}
