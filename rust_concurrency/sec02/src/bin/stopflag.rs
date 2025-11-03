use std::{sync::atomic::{AtomicBool, AtomicI32, Ordering::Relaxed}, thread, time::Duration};

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // 何か処理をするためのスレッドを起動
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed)  {
            some_work();
        }
    });

    // メインスレッドを使ってユーザ入力を受け付ける
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    // バクグランドスレッドに停止する様に知らせる
    STOP.store(true, Relaxed);

    // バックグランドスレッドが終了するまで待つ
    background_thread.join().unwrap();
}

fn some_work() {
    thread::sleep(Duration::from_millis(500));
    println!("working!")
}