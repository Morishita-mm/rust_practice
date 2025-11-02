// 共有参照を通した変更を許可する
// 未定義動作を避けるために、値をコピーして取り出すことと、全体を他の値で置き換えることしかできない
// シングルスレッドでのみ使用可能
use std::cell::Cell;

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        x();    // can reach here←
    }
    println!("before: {}\nafter: {}", before, after);
}

fn x() {
    println!("called x()");
}

// 内容の借用を許可する
// その時点で存在する借用の数を管理するカウンタを保持
// シングルスレッドででのみ使用可能
use std::cell::RefCell;
fn f_refcell(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1); // `Vec`を直接変更可能
}

// 

fn main() {
    let a = Cell::new(10);
    let b = Cell::new(10);
    f(&a, &b);      // don't called x()
    f(&a, &a);    // called x()
}
