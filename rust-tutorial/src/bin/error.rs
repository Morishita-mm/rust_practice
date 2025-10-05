use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("This is rust error sample");
    // 意図的にプログラムをエラーで落とす
    // RUST_BACKTRACE環境変数に0以外の値をセットして、バックとレース出力することでエラー箇所を特定できる
    // panic!("crash and burn");

    // エラーに応じてアクションを行いたい→Result型を使用する
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // エラーの内容が、ファイルが存在しないという形式のエラーだった場合、ファイルを作成する
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There wa a problem opening the file: {:?}",
                error
            )
        },
    };

    // unwrap() : Resultをpanicに変換する（中身がエラーだった場合、パニックを起こす、エラーハンドリングしない）
    // expect() : エラーならパニックにメッセージを渡す、OKなら中身を取り出す、
    // 関数の中でエラーを引き起こす関数を使用する場合、その関数の戻り値もResultにして、依存している関数から帰ってきたエラーをそのまま返すようにすることでエラーを伝搬させることができる
    // ?演算子：エラーだった場合は、直接現在の関数の戻り値としてエラーを返す

    // 理想はpanicではなくResultを返すべきだが、開発初期ではpanicを使用することも考慮すべき
    // panicを出すのは、アプリケーション内で対処できることがなくなったとき、もしくはプログラマの方がコンパイラよりも情報を持っている時（確実に成功するとわかっている時など）

    // DDD的な考え方を適用することもできる
    // 独自型（struct）を使ってプリミティブ型をラップすることで、型の中に制約を押し込むのがRust流
}