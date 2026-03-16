use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        pos: [(usize, usize, usize, usize); n],
    };
    let mut diff = vec![vec![0; 1501]; 1501];

    for (a, b, c, d) in pos {
        diff[a][b] += 1;
        diff[a][d] -= 1;
        diff[c][b] -= 1;
        diff[c][d] += 1;
    }

    // Row-wise prefix sum
    for i in 0..=1500 {
        for j in 1..=1500 {
            diff[i][j] += diff[i][j - 1];
        }
    }
    // Column-wise prefix sum
    for i in 1..=1500 {
        for j in 0..=1500 {
            diff[i][j] += diff[i - 1][j];
        }
    }

    let mut ans = 0;
    for i in 0..1500 {
        for j in 0..1500 {
            if diff[i][j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
