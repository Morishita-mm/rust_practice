// Send
// ある型がSendであれば、安全に別のスレッドに送ることができる

// Sync
// ある型がSyncであれば、別のスレッドと共有できる
// ある型の値への共有参照&TがSendである場合に限り、その型TはSync

use std::{cell::Cell, marker::PhantomData, rc::Rc, sync::Arc, thread};

// X はSendだがSyncではない（Cell<()>がSyncではないため）
struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>,
}

// 明示的にSendとSyncを実装
struct Y {
    p: *mut i32,
}
unsafe impl Send for Y {}
unsafe impl Sync for Y{}

fn main() {
    println!("Hello Sync and Send");
    // let a = Rc::new(123);    // Error
    let a = Arc::new(123);
    thread::spawn(move || {
        dbg!(a);
    });
}