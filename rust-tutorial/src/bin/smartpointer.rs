use std::{ops::Deref, os::fd::BorrowedFd, rc::Rc};
use List::{Cons, Nil};

// Box型に似たものを作ってみる
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(hello: &str) {
    println!("{hello}");
}

// Dropトレイトを実装した構造体を定義してみる
struct CustomSmartPointer {
    data: String,
}

// Rc<T>を使用してみる
// 所有権を複数与えることができる
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// ヒープ領域を開放する際に行われる処理
// ヒープ領域では、OSに対してメモリの確保と解放を依頼する必要がある
impl Drop for CustomSmartPointer {
    // これは直接呼ぶことはできない
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    println!("Smart pointer sample in rust");

    let a: Vec<i32> = vec![1, 2, 3];
    let borrowed_a = &a;
    let b = vec![1, 2, 3];
    // *を使って、借用先の実データに対して直接アクセスしている
    println!("equality: {}", *borrowed_a == b);
    println!("a: {:?}, b: {:?}", a, b);

    let mut moved_a = a;
    let muttably_borrwed_a = &mut moved_a;
    // *を使って、借用元の実際のデータを書き換えている
    *muttably_borrwed_a = vec![1, 2, 3, 4];
    // 元のデータが書き換えられているので以下の出力内容も変化する
    println!("moved_a: {:?}", moved_a);

    // 自作のMyBoxを使用してみる
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // dereference演算子*を使用することで、Boxでラップした中の値にアクセスすることができる
    assert_eq!(5, *y); // *(y.deref())を呼び出している

    let s = MyBox::new(String::from("Hello Smartpointer"));
    // MyBox<T>も、Stringも、Derefトレイトを実装しているので、derefを呼び出すことで内部の値にアクセスすることができる
    // &MyBox<String> -> &String -> &str
    // 二段階の参照外しが内部的に行われている
    hello(&s);

    // CustomSmartpointerを使ってみる
    let c = CustomSmartPointer { data: String::from("my stuff") }; // 後にドロップする
    // drop(d); // cのメモリを解放することができる
    let d = CustomSmartPointer { data: String::from("other stuff") }; // 先にドロップする
    println!("CustomSmartPointer created.");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after createin a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));


}

// コンパイラによる自動的な参照はずし
// T: Deref<Target=U> の時、&T -> &U
// T: DerefMut<Target=U>の時、&mut T -> &mut U
// T: Deref<Target=U>の時、&mut T -> &U

// Rc<T>は読み取り専用にプログラムの複数箇所間でデータを共有させてくれる