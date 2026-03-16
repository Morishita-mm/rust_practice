use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        days: [(usize, usize, usize, usize); n],
    };
    let mut diff = vec![vec![0; w + 2]; h + 2];

    for (a, b, c, d) in days {
        diff[a][b] += 1;
        diff[a][d + 1] -= 1;
        diff[c + 1][b] -= 1;
        diff[c + 1][d + 1] += 1;
    }

    let mut ans = vec![vec![0; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            diff[i][j] = diff[i][j - 1] + diff[i - 1][j] + -diff[i - 1][j - 1] + diff[i][j];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            print!("{}{}", diff[i][j], if j == w { "" } else { " " });
        }
        println!();
    }
}
