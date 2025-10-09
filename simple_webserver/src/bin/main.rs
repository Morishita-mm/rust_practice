extern crate simple_webserver;
use simple_webserver::ThreadPool;

use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    // bind: 新しいTcpListenerインスタンスを返す（リッスンすべきポートに接続する→「ポートに束縛する」ことからbind）
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    // thread::spawnの代わりに、有限個のスレッドを作成できる様にする
    let pool = ThreadPool::new(4);

    // incoming: 一連のストリームを与えるイテレータを返す
    // 単独のストリームがクライアント・サーバ間の開かれた接続を表す
    for stream in listner.incoming() {
        let stream = stream.unwrap();

        // リクエストが来る旅に無制限にスレッドを生成してしまう
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // bufferに読み取ったデータを格納する
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "static/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "static/error/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    // 簡単なレスポンスを作成
    let response = format!("{}{}", status_line, contents);

    // 文字列をバイトデータに変換して、直接そのバイトを送信する
    stream.write(response.as_bytes()).unwrap();
    // flushは待機し、バイトが全て接続に書き込まれるまでプログラムが継続することを保証する
    stream.flush().unwrap();
}
