/*
Mutex:
    可変のデータをスレッド間で共有するために最もよく使われるツール
    Mutex：mutural exclusion（相互排他）の意
    同時にアクセスしようとする別のスレッドを一時的にブロックすることで、対象データへのアクセスを排他的にすること
    Mutexによるデータ保護は、全てのスレッドがそのMutexをロックしている間だけしか保護対象データにアクセスしないことを約束することで実現される
*/

use std::{sync::Mutex, thread};
use std::time::Duration;

fn fn01() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {    // 各スレッド内の動作
                // ロックを獲得する
                // ロック獲得中の動作が実質アトミックとなっている
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    // ガード経由で可変参照を得て更新
                    *guard += 1;
                }
                drop(guard);    // スリープよりも先にガードをドロップ
                // Mutexの時間をできる限り短くすることで、並列性の利点を最大限に活かすことができる
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    // すべてのスレッド終了後、into_inner()で所有権ごと中身を取り出す
    let result = n.into_inner().unwrap();
    println!("result: {}", result);
    assert_eq!(result, 1000);
}


fn main() {
    fn01();
}