use std::collections::HashMap;

fn main() {
    let ans = get_mode_and_mean(vec![1, 2, 3, 2, 2, 3, 1, 1, 1, 1, 1]);
    ans.show_ans();

    println!("{}", pig_latin("sample"));
}

#[derive(Debug)]
struct ModeAndMean {
    mode: i32,
    mean: f64,
}

impl ModeAndMean {
    fn show_ans(&self) {
        println!("mode: {}, mean: {:.4}", self.mode, self.mean);
    }
}

fn get_mode_and_mean(vec: Vec<i32>) -> ModeAndMean {
    let mut num_map: HashMap<i32, i32> = HashMap::new();
    let mut sum = 0;
    for &num in &vec {
        *num_map.entry(num).or_insert(0) += 1;
        sum += num;
    }
    // 最頻値（mode）を求める
    let mode = num_map
        .iter()
        .max_by_key(|entry| entry.1)
        .map(|(k, _)| *k)
        .unwrap();

    // 平均値（mean）を求める
    let mean = sum as f64 / vec.len() as f64;

    ModeAndMean { mode, mean }
}

fn pig_latin(string: &str) -> String {
    match &string[..1] {
        "a" | "i" | "u" | "e" | "o" => format!("{}-hay", string),
        _ => format!("{}-{}ay", &string[1..], &string[0..1]),
    }
}
