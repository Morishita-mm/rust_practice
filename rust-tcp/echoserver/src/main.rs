use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    // 引数がない場合に親切なエラーメッセージを表示
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <address:port>");
        std::process::exit(1);
    }
    let addr = &args[1];
    println!("Server listening on {}", addr);
    echo_server(addr)?;
    Ok(())
}

fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;
    loop {
        let (mut stream, remote_addr) = listener.accept()?;
        println!("New connection: {}", remote_addr);

        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                match stream.read(&mut buffer) {
                    Ok(0) => {
                        println!("Connection closed: {}", remote_addr);
                        return;
                    }
                    Ok(nbytes) => {
                        // 受信データを表示（末尾に改行を追加するか、flushする）
                        print!("Received: {}", str::from_utf8(&buffer[..nbytes]).unwrap_or("Invalid UTF-8"));
                        std::io::stdout().flush().unwrap(); // 画面に即座に表示させる

                        // エコーバック
                        if let Err(e) = stream.write_all(&buffer[..nbytes]) {
                            eprintln!("Failed to write to stream: {}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from stream: {}", e);
                        return;
                    }
                }
            }
        });
    }
}
