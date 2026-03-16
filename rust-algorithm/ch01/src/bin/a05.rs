use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let mut ans = 0;

    for x in 1..=n {
        for y in 1..=n {
            let z = k - x - y;
            if z >= 1 && z <= n {
                ans += 1;
            }
        }
    }

    println!("{}", more_effective(n, k));
    println!("{}", ans);
}

fn more_effective(n: i32, k: i32) -> i64 {
    let mut ans = 0;
    for x in 1..=n {
        let y_min = max(1, k - x - n);
        let y_max = min(n, k - x - 1);
        if y_min <= y_max {
            ans += (y_max - y_min + 1) as i64;
        }
    }
    ans
}
