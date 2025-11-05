use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    X.store(1, Relaxed);
    let t = thread::spawn(f);
    X.store(2, Relaxed);
    t.join().unwrap();

    X.store(3, Relaxed); // これが各スレッドの動作よりも後に実行されることは保証されている

    // 先行発生関係と全変更順序の関係
    let FLAG = &AtomicBool::new(false);
    thread::scope(|s| {
        static mut VALUE: i32 = 0;
        s.spawn(|| unsafe {
            VALUE = 42;
            FLAG.store(true, Relaxed);
        });

        s.spawn(|| unsafe {
            if FLAG.load(Relaxed) {
                let v = VALUE;
                println!("Value:{v}");
            } else {
                println!("Flag is false.");
            }
        });
    });

    // Relaxedメモリオーダリングで順序が保証されないことによって、循環依存する操作がある場合、理論的にややこしくなることがある
    static Y: AtomicI32 = AtomicI32::new(0);
    static Z: AtomicI32 = AtomicI32::new(0);

    let a = thread::spawn(|| {
        let y = Y.load(Relaxed);
        Z.store(y, Relaxed);
    }); 
    let b = thread::spawn(|| {
        let z = Z.load(Relaxed);
        Y.store(z, Relaxed);
    });
    a.join().unwrap();
    b.join().unwrap();

    assert_eq!(Y.load(Relaxed), 0); // 失敗する可能性がある？
    assert_eq!(Z.load(Relaxed), 0); // 失敗する可能性がある？

}

fn f() {
    let x = X.load(Relaxed);
    assert!(x == 1 || x == 2); // 絶対にパニックが発生しない
}

/*
Relaxed オーダリング
先行発生関係をつくらないが、ここのアトミック変数に対する「全変更順序」を保証する
つまり、一つのアトミック変数に対汁すべての犯行は、すべてのスレッドから見て同じ順序で行われる

先行発生関係：
    異なるスレッド間で、処理の順序とデータの可視性を保証し、同期を取るためのルール
    あるスレッドAで起きた操作Xが、別のスレッドBで起きた操作Yよりも前に派生することが保証される
    これにより、Xの前にAが行ったすべての非アトミックな書き込み（通常の変数への書き込みなど）が、Yの後にBが行う処理から可視になる

全変更順序
    特定のアトミック変数に対するすべての書き込み操作が、すべてのスレッドから見て同じ単一の線形的な順序で行われた様に見えることを保証する
    あくまでそのアトミック変数自体の変更履歴に対する保証。アトミック変数の変更順序が混乱することはないが、その変更操作に付随する他のメモリ操作（通常の変数の読み書きなど）の順序や可視性については一切保証しない
*/

fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{a} {b} {c} {d}");
}
