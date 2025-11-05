/*
Sequentially Consistent（逐次実行）
    最も強いメモリオーダリング
    プログラム中に存在するSeqCstオーダリングの全ての操作は全てのスレッドが合意する単一の全順序を構成する
    この全順序は、ここの変数のぜん変更順序と整合している
    実際に必要になることはほとんどない
*/

use std::{sync::atomic::{AtomicBool, Ordering::SeqCst}, thread};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe { S.push('!') };
        }
    });

    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe { S.push('!') };
        }
    });

    a.join().unwrap();
    b.join().unwrap();
}