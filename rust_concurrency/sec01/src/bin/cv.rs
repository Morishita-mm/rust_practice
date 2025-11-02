/*
条件変数
    waitとnotifyを用いてスレッドの操作をする
    アトミックにMutexをアンロックして待機を開始する方法が用意されている

    std:sync::Condvar#wait
        MutexGuardを引数にとり、Mutexをろくしていることを保証する
        Mutexをアン録してスリープする
        起こされた際にはMutexを再度録して新しいMutexGuardを返す
    std::sync::Condvar#notify_one
        待機スレッドの一つだけを起こす
    std::sync::Condvar#notify_all
        すべての待機スレッドを起こす
 */

use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        // キューがからの場合はwaitを呼び出して待機（新しいMutexGuardを取得する）
                        q = not_empty.wait(q).unwrap();
                    }
                };
                // 条件変数not_emptyのロックを解除する
                drop(q);
                dbg!(item);
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
