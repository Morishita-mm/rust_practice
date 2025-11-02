use crossbeam_channel;
use std::{
    sync::Mutex,
    thread,
    time::{Duration, Instant},
};

const NUMS: u64 = 10;
const N_THREADS: usize = 10;
const WAIT_TIME: u64 = 1;

fn complex_process(_: u64) -> bool {
    thread::sleep(Duration::from_secs(WAIT_TIME));
    true
}

fn simple_process(n: u64) -> bool {
        if n < 2 {
        false
    } else if n == 2 {
        true
    } else if n % 2 == 0 {
        false
    } else {
        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

fn main() {
    let start = Instant::now();

    let ans = Mutex::new(Vec::new());
    let nums: Mutex<Vec<u64>> = Mutex::new((1..=NUMS).collect());

    thread::scope(|s| {
        for _ in 0..N_THREADS {
            s.spawn(|| {
                loop {
                    let num_opt = nums.lock().unwrap().pop();
                    if let Some(num) = num_opt {
                        if complex_process(num) {
                            ans.lock().unwrap().push(num);
                        }
                    } else {
                        break;
                    }
                }
            });
        }
    });
    if let Ok(_) = ans.into_inner() {
        let end = start.elapsed();
        println!("multi(simple): {}.{}s", end.as_secs(), end.subsec_millis());
    }

    let start = Instant::now();
    let (tx, rx) = crossbeam_channel::unbounded();

    for n in 1..=NUMS {
        tx.send(n).unwrap();
    }
    drop(tx);

    let mut handles = Vec::new();

    for _ in 0..N_THREADS {
        let rx_clone = rx.clone();
        let handle = thread::spawn(move || {
            let mut local_primes = Vec::new();
            while let Ok(num) = rx_clone.recv() {
                if complex_process(num) {
                    local_primes.push(num);
                }
            }
            local_primes
        });
        handles.push(handle);
    }
    drop(rx);
    let mut total_primes = Vec::new();
    for handle in handles {
        let local_result = handle.join().unwrap();
        total_primes.extend(local_result);
    }

    let end = start.elapsed();
    println!("multi(complex): {}.{}s", end.as_secs(), end.subsec_millis());

    let start = Instant::now();

    let mut nums: Vec<u64> = (1..=NUMS).collect();
    let mut ans = Vec::new();

    loop {
        let num = match nums.pop() {
            Some(n) => n,
            None => break,
        };

        if complex_process(num) {
            ans.push(num);
        }
    }

    let end = start.elapsed();
    println!("single: {}.{}s", end.as_secs(), end.subsec_millis());
}

#[test]
fn test_is_unique() {
    assert_eq!(complex_process(2), true);
    assert_eq!(complex_process(100), false);
    assert_eq!(complex_process(101), true);
}
