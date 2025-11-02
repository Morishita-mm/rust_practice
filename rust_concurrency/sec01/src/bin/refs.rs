use std::{rc::Rc, thread};

fn main() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    // Rcでラップしていることにより、所有権を共有することができる
    // Boxと違い、同じメモリアドレスを参照する変数が作成される
    assert_eq!(a.as_ptr(), b.as_ptr());

    use std::sync::Arc;

    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    thread::spawn(move || dbg!(a)).join().unwrap();
    thread::spawn(move || dbg!(b)).join().unwrap();

    // 変数をシェーディングすることで冗長な記述を減らすことができる
    let a = Arc::new([1, 2, 3]);
    for i in 0..10 {
        let a = a.clone();
        thread::spawn(move || {
            println!("thread {} : {:?}", i, a);
        }).join().unwrap();
    }
}