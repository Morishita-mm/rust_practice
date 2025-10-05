// rustにおいて、ジェネリクスを使用する場合と使用しない場合でパフォーマンスに影響はない
// 単相化（ジェネリクスが使用されていない形）に変換されてから実行される

use std::cmp::PartialOrd;

fn main() {
    println!("Generics sample");

    let numbers_list = vec![45, 39, 25, 100];

    println!("The largest nuber is {}", largest(&numbers_list));

    let numbers_list = vec![45.3, 39.9, 25.1, 600.9, 2.1, 43.1, 100.2];

    println!("The largest nuber is {}", largest(&numbers_list));
}

// Copy：moveするたびに所有権を奪うのではなく、コピー中身をコピーするようになる
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// あらゆる方を受け取れる構造体を使用できる
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// ジェネリックが特定の型の場合の型に対して実装している
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 複数のジェネリックを使用することもできる
struct Point2<T, U> {
    x: T,
    y: U,
}
