/*
スピンロック
すでにロックされているMutexをロックしようとした場合、ビジーループもしくはスピンする、つまり成功するまで何度もロックの取得を試みる
プロセッササイクルを浪費するが、ロックの霊天使を短縮できる場合がある

最小限の構成
共有変数の変更を保護するために使われるのでunsafeを用いてコンパイラがチェックしてくれないコードを書く必要がある
*/

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::*;


pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self { locked: AtomicBool::new(false) }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}
