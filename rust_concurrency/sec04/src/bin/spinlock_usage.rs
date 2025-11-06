use std::thread;
use sec04::unsafe_spinlock::SpinLock;

fn main() {
  let x = SpinLock::new(Vec::new());
  thread::scope(|s| {
    s.spawn(|| x.lock().push(1));
    s.spawn(|| {
      let mut g = x.lock();
      g.push(2);
      g.push(2);
    });
  });
  let g = x.lock();
  assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}
