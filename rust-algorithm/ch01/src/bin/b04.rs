use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let ans = u32::from_str_radix(&n, 2).unwrap();
    println!("{}", ans);
}
