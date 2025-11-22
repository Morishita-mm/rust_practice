use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running on 0.0.0.0:8080");

    // tx: 送信用（クローンして各タスクに渡す）
    // _rx: main関数では使用しない
    let (tx, _rx) = broadcast::channel(10);
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        // 各タスク用に送信機をクローン
        let tx = tx.clone();
        // 新しいタスク用に受信機を作成
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    // パターンA：ユーザーからの入力を受け取った時
                    result = reader.read_line(&mut line) => {
                        let bytes_read = result.unwrap();
                        if bytes_read == 0 {
                            break;
                        }

                        // 全員に送信（メッセージと送信元アドレスのペア
                        tx.send((line.clone(), addr)). unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
