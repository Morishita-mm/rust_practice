fn main() {
    println!("Pattern matching smaple in rust");
    
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        // 上の条件は以下と同じ意味を持つ
        // 通常は範囲チェックと値のキャプチャを同時に行うことができないので、ifガードを使って束縛した値に後から条件をつける必要がある
        // Message::Hello { id } if id >= 3 && id <= 7 => {
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
    
}