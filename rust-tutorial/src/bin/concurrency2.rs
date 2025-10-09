use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    println!("A little bit complex concurrency sample in Rust");

    let m = Mutex::new(5);

    {
        // Mutex#lockでロックを獲得
        // 現在のスレッドをブロックするので、ロックを得られる順番が来るまでは何も作業はしない
        // lockのよびだしがMutexGuardというスマートポインタを返却する
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // 普通のMutexだと、スレッド間で状態を共有できない
    // let counter = Mutex::new(0);

    // Rcはシングルスレッド用で、マルチスレッドには対応していない
    // let counter = Rc::new(Mutex::new(0));

    // 並行な状況で安全に使用できるRcがある
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // RcのcloneをArcに置き換えることで動作する様になる
        let counter = Arc::clone(&counter);

        // counterの所有権が複数のスレッドに跨って存在しようとしているのでエラーが発生する
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // デッドロックを起こしうる典型パターン
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(0));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);

    // スレッド1：先にaをロックしてからbをロック
    let t1 = thread::spawn(move || {
        let lock_a = a1.lock().unwrap();
        println!("thread 1: locked a");
        // 少し待ってスレッド2がbを先にロックする機会をつくる
        thread::sleep(Duration::from_millis(100));
        let lock_b = b1.lock().unwrap();
        println!("thread 1: locked b");

        // 何か作業（到達できれば）
        let _ = (*lock_a, *lock_b);
        println!("thread 1: done");
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);

    // スレッド2：先にbをロックしてからaをロック
    let t2 = thread::spawn(move || {
        let lock_b = b2.lock().unwrap();
        println!("thread 1: locked b");
        // 少し待ってスレッド1がaを先にロックする機会をつくる
        thread::sleep(Duration::from_millis(100));
        let lock_a = a2.lock().unwrap();
        println!("thread 1: locked a");

        // 何か作業（到達できれば）
        let _ = (*lock_a, *lock_b);
        println!("thread 2: done");
    });

    t1.join().unwrap();
    t2.join().unwrap();

    // デッドロックが発生して到達できない
    println!("finished");
}

/*
Mutex<T>を使用することで、その内部にある値っへの可変参照を得ることができる
これは、RefCell<T>を使用してRc<T>の内容を可変化できる様にした様に、内部化編成を提供している
しかし、Mutex<T>を使用する際にあらゆる種類のロジックエラーからコンパイルは保護してくれない
互いに競合するリソースに対してアクセスしようとしてデッドロックを生成するリスクを持っているため、適切に使用する必要がある
*/
