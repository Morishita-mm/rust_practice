/*
Release / Acquireはついで使用され、スレッド間の先行発生関係を形成する
Releaseはストア操作に、Acquireはロード操作に適用される
AcqRelは、ロード部分はAcquire、ストア部分はReleaseが適用される（更新操作や比較交換操作などに適用した場合に使用する）
*/
use std::{
    sync::atomic::{
        AtomicBool, AtomicU64,
        Ordering::{Acquire, Relaxed, Release},
    },
    thread,
    time::Duration,
};

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release); // このストアよりも前に起こったことはすべて..
    });
    while !READY.load(Acquire) {
        // ..このロードがtureを読み込んだ後は観測できる
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}", DATA.load(Relaxed));

    // 通常の非アトミック型をデータを格納する変数に用いると、コンパイラが拒否する
    // unsafeコードを使って、コンパイラに対してルールを破っていないことを約束しなければならない

    static mut DATA2: u64 = 0;
    READY.store(false, Relaxed);

    thread::spawn(|| {
        // 安全性：まだREADYフラグをセットしていないので誰かがDATAにアクセスすることはない
        unsafe { DATA2 = 123 };
        READY.store(true, Release); // このストアよりも前に起こったことはすべて..
    });
    while !READY.load(Acquire) {
        // .. このロードがtrueを読子mんだ後は観測できる
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    // 安全性：READYが真なので、誰かがDATAを変更することはない
    println!("{}", unsafe { DATA2 });
}

use std::sync::atomic::AtomicPtr;

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            // 安全性：pは直前のBox::into_rowで作ったものなので他のスレッドと共有されていることはない
            // 比較交換失敗次の動作（読み込み次にはヌルポインタだったが、書き込もうとした時にはすでに初期値が他のスレッドによって書き込まれている場合）
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // 安全性：pはヌルではなく適切に初期化した値を指している
    unsafe { &*p }
}

struct Data {
    id: u64,
}

fn generate_data() -> Data {
    Data { id: 100 }
}

/*
メモ
Mutexのアンロックとその後のロックとの間に先行発生関係がある

AtomicPtr<T>はTへのポインタである *mut T のアトミック版
初期状態を指すプレースホルダとしてヌルポインタを用い、新しく確保して完全に初期化されたTへのポインタと、アトミックな比較交換操作を用いて置き換える
*/
