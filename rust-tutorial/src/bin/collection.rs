use std::collections::HashMap;

// 1つのVecに複数の型を入れる方法として、Enumでラップする方法がある
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let mut v = vec![100, 32, 57];
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &mut v {
        *i += 50;
    }

    println!("{:#?}", v);

    // String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("too");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // 1文字ずつ取り出す
    for c in s.chars() {
        println!("{c}");
    }

    // 各バイトをそのまま取り出す
    for b in s.bytes() {
        println!("{b}");
    }

    // HashMap
    let text = String::from("Hello world wonderful world");
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", map);
}
