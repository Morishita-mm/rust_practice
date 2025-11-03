use std::{sync::atomic::{AtomicUsize, Ordering::Relaxed}, thread, time::Duration};

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        // 1つのバックグランドスレッドで100個のアイテムをすべて処理する
        s.spawn(|| {
            for i in 0..10 {
                process_item(i);    // この処理にある程度時間がかかるものとする
                num_done.store(i + 1, Relaxed);
                // num_done.fetch_add(1, Relaxed); // これでも動作する
                main_thread.unpark();   // メインスレッドを起こす
            }
        });

        // メインスレッドは、毎秒更新する
        loop {
            let n = num_done.load(Relaxed);
            if n == 10 { break; }
            println!("Working.. {n}/10 done");
            // thread::sleep(Duration::from_secs(1));
            // park_timeout()で最大1秒まつが、状態が更新されれば即座にユーザに通知する
            thread::park_timeout(Duration::from_secs(1));
        }
    })
}

fn process_item(_: usize) {
    thread::sleep(Duration::from_millis(500));
}

// 遅延初期化
use std::sync::atomic::AtomicU64;

// 最初にこの関数を実行したものだけがcalculate_x()を呼び出す
// calculate_xの実行時間によっては複数のスレッドが呼び出しを同時の行なってしまう可能性があるが、標準ライブラリでこれを防ぐ機能が提供されているので自分で実装する必要はない
fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

fn calculate_x() -> u64{
    todo!();
}