use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    };

    let (a_first, a_second) = a.split_at(n / 2);

    let mut p = get_all_sums(a_first);
    let q = get_all_sums(a_second);

    p.sort();

    for &val_q in &q {
        let target = k - val_q;

        if p.binary_search(&target).is_ok() {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn get_all_sums(cards: &[i64]) -> Vec<i64> {
    let mut sums = vec![0];
    for &card in cards {
        let mut next_sums = Vec::with_capacity(sums.len() * 2);
        for &s in &sums {
            next_sums.push(s);
            next_sums.push(s + card);
        }
        sums = next_sums;
    }
    sums
}
