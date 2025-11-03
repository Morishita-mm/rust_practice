use std::{sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering::Relaxed}, thread, time::{Duration, Instant}};

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // 4つのバックグランドスレッドがそれぞれ25アイテム、計100アイテムを処理する
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t*25+i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        // メインスレッドは更新された状態を毎秒表示する
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 { break; }
            if n == 0 {
                println!("Working.. nothing done yes.");
            } else {
                println!(
                    "Working.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time,
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done");
}

fn process_item(i: i32) {
    let sleep_time= rand::random::<u8>() as u64 * 10;
    thread::sleep(Duration::from_millis(sleep_time as u64));
}

// オーバーフローしてしまう
fn allocate_new_id_v1() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Relaxed)
}

// 方法1：assertでパニックする前に別スレッドによってインクリメントされて、そのスレッドがパニックする前にさらに別スレッドがインクリメントして...をクエリ返すとオーバーフローが発生してしまう可能性がある
// 方法2：std::process::abort()でプログラム全体を停止してしまう
fn allocate_new_id_v2() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    // std::process::abort();
    assert!(id < 1000, "too many IDs");
    id
}

// 方法3：fetch_subを使ってデクリメントする
fn allocate_new_id_v3() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Relaxed);
        panic!("too many IDs!");
    }
    id
}