#[derive(Debug, Clone)]
struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

// ジェネリクスを使用していてレーターを作成すると、トレイとの使用時に必ずジェネリクス宣言をする必要がある(どの型に対する実装を呼び出すのかを明記しないと予期しないものがよばれる可能性がある)（下記の例だと、u32の実装を呼びたいのにi32に対する実装が呼ばれてしまっている）
trait MyIterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl MyIterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        if self.count > 10 {
            return None;
        }

        let count = self.count;
        self.count += 1;
        Some(count)
    }
}

impl MyIterator<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        if self.count > 10 {
            return None;
        }

        let count = self.count;
        self.count += 1;
        Some(-(count as i32))
    }
}

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count > 10 {
//             return None;
//         }

//         let count = self.count;
//         self.count += 1;
//         Some(count)
//     }
// }

use std::fmt;

struct Wrapper(Vec<String>);

// 外部の型を新しい新しい型でラップすることで実装を上書きすることができる
// 今回の場合、Vec<Strint>型のデフォルト表示の方法を変更している
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct Meter(u32);
struct MilliMeter(u32);

fn rectangle_area(width: Meter, height: Meter) -> u32 {
    width.0 * height.0
}

// Meterへの変換を行うトレイとはMeterに対して実装する
// MilliMeterに対して実装しているわけではない
// pub trait From<T> { fn from(value: T) -> Self; }
impl From<MilliMeter> for Meter {
    fn from(mm: MilliMeter) -> Meter {
        assert!(mm.0 % 1000 == 0);
        Meter(mm.0 / 1000)
    }
}

fn main() {
    println!("Advanced trait sample in rust");

    let mut counter = Counter::new();
    assert_eq!(Some(0), <Counter as MyIterator<u32>>::next(&mut counter));
    assert_eq!(Some(1), <Counter as MyIterator<u32>>::next(&mut counter));
    assert_eq!(Some(2), <Counter as MyIterator<u32>>::next(&mut counter));

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let ten_meter = Meter(10);
    let twenty_millimeter = MilliMeter(2000);
    println!(
        "Area of rectangle is {} square meter",
        // ちゃんとコンパイラ側で弾いてくれる
        // rectangle_area(ten_meter, twenty_centimeter)
        // Fromトレイトを実装することで相互に変換することができる様になっている
        rectangle_area(ten_meter, twenty_millimeter.into())
    );
}

// NewTypePatternを使用する理由
// 型安全性のため
// 内部の型の実装を隠蔽して特定の操作しかできない様にすることで型の安全性を高める
// プリミティブ型をラッパークラスでラップしようねっていう話の延長線上にある
