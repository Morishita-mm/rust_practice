/*
 * 以下の実装だといくつか問題がある
 * 1. インタフェースがわかりづらく、間違った方法で使えてしまう
 * 2. sendを複数回呼び出すと、データ競合が起こる可能性がある
 * 3. Dropが実装されておらず、Channelをドロップしても中のUnsafeCellがリークする可能性がある
 */

use std::sync::Arc;
use std::sync::atomic::Ordering::*;
use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicBool};

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool, // メッセージが消費可能かどうか
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });
    (Sender { channel: a.clone() }, Receiver { channel: a })
}

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}
pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    // このメソッドはパニックしない
    // selfは参照ではなく値を受け取っている
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release);
    }
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    // selfは参照ではなく実態を受け取っている
    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Acquire) {
            panic!("no message available!");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}
