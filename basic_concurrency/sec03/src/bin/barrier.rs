use std::sync::{Arc, Barrier}; // ❶
use std::thread;

fn main() {
    // スレッドハンドラを保存するベクタ
    let mut v = Vec::new(); // ❷

    // 10スレッド分のバリア同期をArcで包む
    let barrier = Arc::new(Barrier::new(10)); // ❸

    // 10スレッド起動
    for _ in 0..10 {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            b.wait(); // バリア同期 ❹
            println!("finished barrier");
        });
        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }
}