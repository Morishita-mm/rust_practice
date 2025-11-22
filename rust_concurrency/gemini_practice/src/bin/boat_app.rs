use std::{collections::HashMap, sync::{Arc, Mutex}};

use tokio::{io::{AsyncBufReadExt, BufReader, AsyncWriteExt}, net::TcpListener, sync::broadcast};

type VotingBox = Arc<Mutex<HashMap<String, u32>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Voting Server running on 0.0.0.0:8080");

    let voting_box = VotingBox::new(Mutex::new(HashMap::new()));
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New voter: {}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let voting_box = Arc::clone(&voting_box);

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        let bytes = result.unwrap();
                        if bytes == 0 { break; }

                        let team_name = line.trim().to_string();

                        if !team_name.is_empty() {
                            let message = {
                                let mut box_guard = voting_box.lock().unwrap();
                                *box_guard.entry(team_name).or_insert(0) += 1;

                                let mut msg = String::from("現在の集計: ");
                                for(team, count) in box_guard.iter() {
                                    msg.push_str(&format!("{}: {}票, ", team, count));
                                }
                                msg.push('\n');
                                msg
                            };

                            tx.send(message).unwrap();
                        }
                        line.clear();
                    }

                    result = rx.recv() => {
                        let msg = result.unwrap();
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }

        });
    }
}