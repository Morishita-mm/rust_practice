use std::{cell::UnsafeCell, sync::atomic::AtomicBool};
use std::sync::atomic::Ordering::*;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,   // Syncを実装していない
}

// 複数スレッドからの同時アクセスを想定するなら T: Sync を追加で要求する必要がある
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    // pub fn lock(&self) -> &mut T {
    //     while self.locked.swap(true, Acquire) {
    //         std::hint::spin_loop();
    //     }
    //     // UnsafeCell.get() -> &mut T
    //     unsafe { &mut *self.value.get() }
    // }

    pub fn lock(&self) -> Guard<'_,T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }

    // 安全性：lock()が返した &mut T はなくなっていなければならない
    // （Tに対する参照をどこかにとっておいちゃだめ）
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

//
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Guard<'_, T> {
  type Target = T;
  fn deref(&self) -> &T {
    // 安全性：このガードが存在すること自体が、ロックを排他的に取得したことを保証する
    unsafe { &*self.lock.value.get() }
  }
}

impl<T> DerefMut for Guard<'_, T> {
  fn deref_mut(&mut self) -> &mut T {
    // 安全性：このガードが存在すること自体が、ロックを排他的に取得したことを保証する
     unsafe { &mut *self.lock.value.get() }
  }
}

//　GuardがTがSyncの場合にだけSyncとなることを明示的に示しておかないと、複数のスレッドが間違って一つのGuard<T>を共有してしまい、TがSyncでなくても同一のTに対してアクセスできてしまう
unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

impl<T> Drop for Guard<'_, T> {
  fn drop(&mut self) {
    self.lock.locked.store(false, Release);
  }
}
