use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let flag = val.read().unwrap(); // ❶ Readロック獲得
        if *flag {
            *val.write().unwrap() = false; // ❷ Readロック獲得中にWriteロック獲得。デッドロック発生
            println!("flag is true");
        }
    });

    t.join().unwrap();
}

//デッドロックの回避策
/*
fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let flag = *val.read().unwrap(); // ❶ Readロックを獲得し値を取り出した後即座にロック解放
        if flag {
            *val.write().unwrap() = false; // ❷
            println!("flag is true");
        }
    });

    t.join().unwrap();
}
*/