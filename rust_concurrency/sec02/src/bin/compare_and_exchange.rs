use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicU32, AtomicU64};
use std::thread;
use std::time::Duration;

fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => current = v, // 値をロードしてからここに到達するまでに別スレッドで値が書き換えら絵rていることを意味する
        }
    }
}

fn main() {
    let target = &AtomicU32::new(0);
}

// オーバーフローを起こさない一意なIDの発行
fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed);

    loop {
        assert!(id < 1000, "Too many IDs!");
        // 比較と交換がアトミックだからこそ実現できる
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}

// fetch_updateを用いてさらに実装を簡略化できる
// 　比較交換するループパターンを実装するためのメソッド
fn allocate_new_id_v2() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID
        .fetch_update(Relaxed, Relaxed, |n| n.checked_add(1))
        .expect("too many IDs!")
}

fn get_ky() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);
    if key == 0 {
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    } else {
        key
    }
}

fn generate_random_key() -> u64 {
    let sleep_time = rand::random::<u8>() as u64 * 10;
    thread::sleep(Duration::from_millis(sleep_time as u64));
    sleep_time
}
