use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n],
    }

    // let ans = a.binary_search(&x).unwrap();
    let ans = my_binary_search(x, &a).unwrap();

    println!("{:?}", ans + 1);
}

fn my_binary_search(t: i32, s: &[i32]) -> Result<usize, usize> {
    let mut size = s.len();
    if size == 0 {
        return Err(0);
    }
    let mut base = 0usize;

    while size > 1 {
        let half = size / 2;
        let mid = base + half;

        if s[mid] <= t {
            base = mid;
        }
        size -= half;
    }

    if s[base] == t {
        Ok(base)
    } else {
        Err(base + (s[base] < t) as usize)
    }
}
