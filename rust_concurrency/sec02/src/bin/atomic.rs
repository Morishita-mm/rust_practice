use std::sync::atomic::{AtomicI32, Ordering::Relaxed};

fn main() {
    let a = AtomicI32::new(100);
    let b = a.fetch_add(23, Relaxed);   // fetch_addの戻り値は古い値
    let c = a.load(Relaxed);

    assert_eq!(b, 100);
    assert_eq!(c, 123);
}


/*
fetch_addとfetch_subはオーバーフローに対してラップする
    インクリメントの結果、値がその方の最大値よりも大きくなってしまった場合には、ラップアラウンドしてその型の最小値になる
    通常の整数値では、オーバーフローに対してデバッグモードではパニックする
*/