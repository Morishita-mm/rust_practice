// iter()メソッドを使用するには、Iteratorトレイトを実装する必要がある
// Iteratorトレイトでは、next()メソッドが定義されておりOption<Self::Item>を返すメソッドである

/*
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
*/

#[derive(PartialEq, Debug)]
struct Shoe {
size: u32,
style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

fn main() {
    println!("Iterator sample in rust");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![1, 2, 3];
    let v2: Vec<_> = v2.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
