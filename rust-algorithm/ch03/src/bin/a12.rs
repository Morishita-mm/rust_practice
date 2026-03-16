use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let check = |t: usize| -> bool {
        let mut total = 0;
        for &time_per_sheet in &a {
            total += t / time_per_sheet;
            if total >= k {
                return true;
            }
        }
        total >= k
    };

    let mut left = 1;
    let mut right = 1_000_000_000;

    while left < right {
        let mid = left + (right - left) / 2;

        if check(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    println!("{left}");
}
