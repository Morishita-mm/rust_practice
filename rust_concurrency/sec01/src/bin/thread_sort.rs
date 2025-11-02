use std::{thread, time::Duration};

fn main() {
    let nums = vec![12, 3, 2, 1, 100, 32, 59];
    thread::scope(|s| {
        for num in nums {
            s.spawn(move || {
                thread::sleep(Duration::from_millis(num));
                println!("{num}");
            });
        }
    })
}
