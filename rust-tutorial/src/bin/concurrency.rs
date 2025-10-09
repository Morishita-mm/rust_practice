// 実行中のプログラムのコードはプロセスではしる
// プログラム内の計算を複数のスレッドに分けるとパフォーマンスの改善が見られることがあるが、複雑さもます
// ・データやリソースの競合
// ・デッドロック
// ・再現や修正が困難なバグ

// OSから渡されたNこのスレッドを擬似的にM個のスレッドに分割して管理することが多いが、
// Rustでは、1：1スレッドの実装のみを提供している
// M:Nスレッドの実装をしたクレーともある

use std::f32::consts::PI;
use std::thread;
use std::time::Duration;
// multiple producer, ingle consumer
// 複数の送信側と、その値を消費する一つの受信側を持つことができる
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // threaed::spawnの戻り値から呼び出している
    // JoinHandleはそのjoinメソッドを呼び出した時にスレッドの終了を待つ所有された値
    // mainスレッドが終了する前にspawnスレッドが完了するのを保証している
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // Rustはvのキャプチャ方法を推論し、println!はvへの参照のみを必要とするのでvを借用しようとするが、コンパイラは立ち上げたスレッドがどの期間走るのかわからないので、vへの参照が常に有効であるか把握できない
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // これはクロージャ内に所有権が奪われているのでコンパイルが通らない
    // drop(v);

    handle.join().unwrap();

    // メッセージ受け渡しを使ってスレッド間でデータを転送する
    // tx: 転送機　rx: 受信機
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // 送信した値はこのスレッド内での所有権を失う
        // println!("{}", val); //valの所有権がないのでこれはエラー！
    });

    // recv()：メインスレッドをブロックし、値がチャンネルを流れてくるまで待機する
    let received = rx.recv().unwrap();
    // recv_try()：メインスレッドをブロックせずに即座にRexult<T, E>が返される
    println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}
