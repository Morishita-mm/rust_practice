use proconio::input;

fn main() {
    input! {
        n: f64,
    }

    let mut left = 0.0;
    let mut right = 100.0;

    for _ in 0..100 {
        let mid = (left + right) / 2.0;

        if mid * mid * mid + mid < n {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{:.12}", left);
}
