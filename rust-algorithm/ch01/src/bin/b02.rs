use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    for n in a..b + 1 {
        if 100 % n == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
