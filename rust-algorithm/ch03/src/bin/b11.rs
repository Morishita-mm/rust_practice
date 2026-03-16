use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        q: usize,
    }

    a.sort();
    for _ in 0..q {
        input! {
            x: i32,
        }
        println!("{}", a.partition_point(|&val| val < x));
    }
}
