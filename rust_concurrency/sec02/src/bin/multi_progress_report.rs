use std::{sync::atomic::{AtomicUsize, Ordering::Relaxed}, thread::{self, sleep}, time::Duration};
use rand::Rng;

fn main() {
    let num_done = &AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        // 4つのバックグラウンドスレッドがそれぞれ25アイテム、合計100アイテムを処理する
        for t in 0..4 {
            let main_thread = main_thread.clone();
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);   // この処理に時間がかかると想定
                    num_done.fetch_add(1, Relaxed); // 加算後の値には興味がない
                    main_thread.unpark();
                }
            });
        }

        // メインスレッドは更新された状態を毎秒表示する
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 { break; }
            println!("Working.. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn process_item(i: i32) {
    let sleep_time= rand::random::<u8>() as u64 * 10;
    thread::sleep(Duration::from_millis(sleep_time as u64));
}