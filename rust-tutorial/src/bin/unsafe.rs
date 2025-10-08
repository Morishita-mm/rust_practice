// unsafe : コンパイラではなく人間が動作を保証する
// unsafeで修飾されている関数はunsafeブロックの中でしか使用できない
// 可変で静的な変数を変更しようとするときもunsafeをつける必要がある
//      マルチスレッドに対応した時に値に矛盾が生じる可能性があるため

use core::slice;

// unsafeなメソッド
unsafe fn dangerous() {}

// 安全な関数でラップして内部ではunsafeの範囲を可能な限り狭める
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    // unsafeを使用する際は、使用の前にアサーションでチェックして、unsafeではないという根拠を持っておくとわかりやすそう
    // デバッグ時にはassert!で確認して、本番環境ではカスタムエラーを返したり、panic!マクロでエラーメッセージを詳細かする必要があることに注意
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

fn main() {
    println!("Unsafe Rust smaple");

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // コンパイラはポインタが指している値の先に有効な値があることを保証できない
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // unsafeブロックでしかunsafeなメソッドを呼び出せない
    unsafe {
        dangerous();
    }
}
