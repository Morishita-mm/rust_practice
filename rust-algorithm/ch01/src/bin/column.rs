use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
    }

    let mut exists = false;

    for i in 0..(1 << n) {
        let mut sum = 0;

        for j in 0..n {
            if (i >> j) & 1 == 1 {
                sum += a[j];
            }
        }

        if sum == k {
            exists = true;
            break;
        }
    }

    if exists {
        println!("Yes");
    } else {
        println!("No");
    }
}
