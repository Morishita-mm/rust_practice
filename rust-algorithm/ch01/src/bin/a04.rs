use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    // 9 から 0 まで逆順にループ
    // (0..10) は 0, 1, ..., 9 を生成し、.rev() で 9, 8, ..., 0 になる
    for x in (0..10).rev() {
        let wari = 1 << x; // 2 の x 乗
        // print! は改行なしで出力する
        print!("{}", (n / wari) % 2);
    }

    // 最後に改行を出力
    println!();
}
