use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
        queries: [(usize, usize); q],
    }

    let psum: Vec<i32> = std::iter::once(0)
        .chain(a.iter().scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        }))
        .collect();

    for (l, r) in queries {
        println!("{}", psum[r] - psum[l - 1]);
    }
}
