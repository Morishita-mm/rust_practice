// 関数を受け取る関数、高階関数ってことだと思う
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice_closure<F>(f: F, arg: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

// 戻り値はBoxでラップしなくても返るクロージャは同じ
// コンパイル時に戻り値の型を特定できるため！
fn get_add_n_closure(n: usize) -> impl Fn(usize) -> usize {
    move |x| x + n
}

// 戻るクロージャが異なるため、Boxでラップしてやる必要がある
// コンパイル時に戻り値の型を特定できないため！
fn get_add_or_multi_closure(n: usize) -> Box<dyn Fn(usize) -> usize> {
    if n > 10 {
        Box::new(move |x| x + n)
    } else {
        Box::new(move |x| x * n)
    }
}

fn main() {
    println!("Advanced method and closure sample in rust");

    let added = 20;
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    // クロージャを渡すことも可能
    // 引数の型がfnの場合、環境のキャプチャはできない
    // println!("The answer is: {}", do_twice(|v| v + added, 5)); // コンパイルエラー
    // 引数にfnポインタではなく、Fn, FnMut, FnOnceトレイトを境界として持つジェネリクスを使用することで、環境のキャプチャが行えるようになる
    println!("The answer is: {}", do_twice_closure(|v| v + added, 5));

    let f2 = get_add_n_closure(2);
    let f5 = get_add_n_closure(5);
    println!("f2(3) = {}", f2(3));
    println!("f5(3) = {}", f5(3));

    let f2 = get_add_or_multi_closure(2);
    let f20 = get_add_or_multi_closure(20);
    assert_eq!(f2(3), 6);
    assert_eq!(f20(3), 23);
}