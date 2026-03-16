use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        points: [(usize, usize); n],
        q: usize,
    }

    let mut s = vec![vec![0; 1501]; 1501];
    for (x, y) in points {
        s[x][y] += 1;
    }

    for i in 1..=1500 {
        for j in 1..=1500 {
            s[i][j] += s[i - 1][j] + s[i][j - 1] - s[i - 1][j - 1];
        }
    }

    for _ in 0..q {
        input! {
            a: usize, b: usize, c: usize, d: usize,
        }
        let ans = s[c][d] - s[a - 1][d] - s[c][b - 1] + s[a - 1][b - 1];
        println!("{ans}");
    }
}
