use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let mut ans = 0;
    let mut j = 0;

    for i in 0..n {
        while j < n && s[j + 1] - s[i] <= k {
            j += 1;
        }
        ans += (j - i) as i64;
    }
    println!("{ans}");
}
