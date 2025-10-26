use crate::semaphore::Semaphore;
use std::collections::LinkedList;
use std::sync::{Arc, Condvar, Mutex};

// 送信端のための型 ❶
#[derive(Clone)]
pub struct Sender<T> {
    sem: Arc<Semaphore>, // 有限性を実現するセマフォ
    buf: Arc<Mutex<LinkedList<T>>>, // キュー
    cond: Arc<Condvar>, // 読み込み側の条件変数
}

impl<T: Send> Sender<T> { // ❷
    // 送信関数
    pub fn send(&self, data: T) {
        self.sem.wait(); // キューの最大値に到達したら待機 ❸
        let mut buf = self.buf.lock().unwrap();
        buf.push_back(data); // エンキュー
        self.cond.notify_one(); // 読み込み側へ通知 ❹
    }
}

// 受信端のための型 ❶
pub struct Receiver<T> {
    sem: Arc<Semaphore>, // 有限性を実現するセマフォ
    buf: Arc<Mutex<LinkedList<T>>>, // キュー
    cond: Arc<Condvar>, // 読み込み側の条件変数
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> T {
        let mut buf = self.buf.lock().unwrap();
        loop {
            // キューから取り出し ❷
            if let Some(data) = buf.pop_front() {
                self.sem.post(); // ❸
                return data;
            }
            // 空の場合待機 ❹
            buf = self.cond.wait(buf).unwrap();
        }
    }
}

pub fn channel<T>(max: isize) -> (Sender<T>, Receiver<T>) {
    assert!(max > 0);
    let sem = Arc::new(Semaphore::new(max));
    let buf = Arc::new(Mutex::new(LinkedList::new()));
    let cond = Arc::new(Condvar::new());
    let tx = Sender {
        sem: sem.clone(),
        buf: buf.clone(),
        cond: cond.clone(),
    };
    let rx = Receiver { sem, buf, cond };
    (tx, rx)
}