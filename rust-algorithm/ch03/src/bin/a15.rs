use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };

    let mut b = a.clone();
    b.sort();
    b.dedup();

    let ans: Vec<String> = a
        .iter()
        .map(|&x| (b.binary_search(&x).unwrap() + 1).to_string())
        .collect();
    println!("{}", ans.join(" "));
}
