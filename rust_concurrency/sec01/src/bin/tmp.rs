use std::{
    collections::HashMap,
    sync::Mutex,
    thread,
    time::{Duration, Instant}, // 実行時間計測用
};

const WAIT_MILLIS: u64 = 10; // 待機時間

// 複雑な処理をする関数（擬似的にスリープで再現）
fn complex_process(n: u64) -> bool {
    thread::sleep(Duration::from_millis(WAIT_MILLIS));
    n % 2 == 0
}

// 単純な処理をする関数（奇数ならfalse、偶数ならtrueを返す）
fn simple_process(n: u64) -> bool {
    n % 2 == 0
}

const N_ITEROS: u16 = 10;

// 並行処理
fn main() {
    let mut result: HashMap<(bool, i32), Vec<u128>> = HashMap::new();
    let thread_list = vec![1, 2, 3, 5, 10, 20];
    let comp_list = vec![true, false];

    for is_comp in comp_list {
        for n_threads in &thread_list {
            for _ in 0..N_ITEROS {
                let res = if *n_threads == 1 {
                    single(is_comp)
                } else {
                    multi(is_comp, *n_threads)
                };
                let key = (is_comp, *n_threads);
                result
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(res.as_millis());
            }
        }
    }

    show_result(&result);
}

fn multi(is_comp: bool, n_threads: i32) -> Duration {
    let nums: u64 = if is_comp { 10 } else { 1000000 }; // 何回処理を行うか
    let start = Instant::now(); // 計測開始

    let ans = Mutex::new(Vec::new());
    let nums: Mutex<Vec<u64>> = Mutex::new((1..=nums).collect());

    thread::scope(|s| {
        for _ in 0..n_threads {
            s.spawn(|| {
                loop {
                    let num_opt = nums.lock().unwrap().pop();
                    let num = match num_opt {
                        Some(n) => n,
                        None => break,
                    };

                    let keep = if is_comp {
                        complex_process(num)
                    } else {
                        simple_process(num)
                    };

                    if keep {
                        ans.lock().unwrap().push(num);
                    }
                }
            });
        }
    });

    if let Ok(final_ans) = ans.into_inner() {
        // 計測終了
        let end = start.elapsed();
        return end;
    } else {
        panic!("Something went wrong");
    }
}

fn single(is_comp: bool) -> Duration {
    let nums: u64 = if is_comp { 10 } else { 1000000 }; // 何回処理を行うか

    let start = Instant::now(); // 計測開始

    let mut nums: Vec<u64> = (1..=nums).collect();
    let mut ans = Vec::new();

    loop {
        let num = match nums.pop() {
            Some(n) => n,
            None => break,
        };

        let keep = if is_comp {
            complex_process(num)
        } else {
            simple_process(num)
        };

        if keep {
            ans.push(num);
        }
    }

    return start.elapsed();
}

fn show_result(result: &HashMap<(bool, i32), Vec<u128>>) {
    let mut keys: Vec<&(bool, i32)> = result.keys().collect();

    keys.sort_by_key(|k| (k.1, k.0));

    println!(
        "--- 実行時間計測結果 (平均 {} 回 / スレッド数昇順) ---",
        N_ITEROS
    );
    println!("| 処理内容 | スレッド数 | 平均時間 (ms) |");
    println!("|----------|------------|---------------|");

    for key in keys {
        let (is_comp, n_threads) = *key;
        let times = result.get(key).unwrap();

        // 合計と平均の計算
        let sum: u128 = times.iter().sum();
        let count = times.len() as u128;

        let average_ms = if count > 0 {
            (sum as f64) / (count as f64)
        } else {
            0.0
        };

        let comp_label = if is_comp {
            "複雑な処理"
        } else {
            "単純な処理"
        };

        println!("| {} | {:02} | {:5.3} |", comp_label, n_threads, average_ms);
    }
    println!("--------------------------------------");
}
