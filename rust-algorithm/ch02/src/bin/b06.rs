use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
        q: usize,
        queries: [(usize, usize); q],
    }

    let psum: Vec<i32> = std::iter::once(0)
        .chain(a.iter().scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        }))
        .collect();

    for (l, r) in queries {
        let total = (r - l + 1) as i32;
        let wins = psum[r] - psum[l - 1];
        let loses = total - wins;

        if wins > loses {
            println!("win");
        } else if wins < loses {
            println!("lose");
        } else {
            println!("draw");
        }
    }
}
