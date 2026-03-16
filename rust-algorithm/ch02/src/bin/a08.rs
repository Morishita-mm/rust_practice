use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [[i32; w]; h],
        q: usize,
    }
    let mut s = vec![vec![0; w + 1]; h + 1];

    for i in 1..=h {
        for j in 1..=w {
            s[i][j] = s[i - 1][j] + s[i][j - 1] - s[i - 1][j - 1] + grid[i - 1][j - 1];
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
