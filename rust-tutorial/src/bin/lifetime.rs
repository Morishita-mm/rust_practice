fn main() {
    println!("Lifetime sample in rust");

    // string1 と string2のライフタイムは異なる
    let string1 = String::from("long string is long");

    {
        // longest()メソッドを使用することで、resultのライフタイムがこのブロック内に限定される
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("Thre longest string is {}", result);
    }

}

// より短いライフタイムが採用される
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 構造体自身のライフタイムと、中身の値のライフタイムを揃えることができる
struct SomeStruct<'a> {
    part: &'a str,
}