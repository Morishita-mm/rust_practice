// 仮のプロジェクト名 (コンパイルする際のプロジェクト名) を使って
// lib.rs で公開した channel モジュール内の channel 関数をインポート
// 実際にはあなたのプロジェクト名に合わせて 'concurrency_examples' を変更してください
use sec03::channel::channel;

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 8;

fn main() {
    // 最大バッファサイズ4のチャネルを作成
    let (tx, rx) = channel(4);
    let mut v = Vec::new();

    // 受信用スレッド (コンシューマー)
    let t = std::thread::spawn(move || {
        let mut cnt = 0;
        // 送信される総データ数 (NUM_THREADS * NUM_LOOP) に等しくなるまで受信
        while cnt < NUM_THREADS * NUM_LOOP {
            let (i, j) = rx.recv();
            // 受信ログは大量に出力されるため、コメントアウトを推奨
            println!("recv: n = ({}, {})", i, j);
            cnt += 1;
        }
        println!("\n--- [Receiver] Total received: {} ---", cnt);
    });

    v.push(t);

    // 送信用スレッド (プロデューサー)
    for i in 0..NUM_THREADS {
        let tx0 = tx.clone();
        let t = std::thread::spawn(move || {
            for j in 0..NUM_LOOP {
                // 送信ログは大量に出力されるため、コメントアウトを推奨
                println!("send: n = ({}, {})", i, j);
                tx0.send((i, j));
            }
            println!("- [Sender {}] Finished sending {} items.", i, NUM_LOOP);
        });
        v.push(t);
    }

    // すべてのスレッドの完了を待機
    for t in v {
        t.join().unwrap();
    }
    
    println!("All threads finished. The channel test is complete.");
}
