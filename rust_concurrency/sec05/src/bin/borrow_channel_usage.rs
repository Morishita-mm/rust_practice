use std::thread;

use sec05::borrow_channel::Channel;

fn main() {
    // スコープの前にチャネルを作成することで、SenderとReceiverのライフタイムよりもチャネルのライフタイムが長いことを示している
    let mut channel = Channel::new();

    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        let t = thread::current();
        s.spawn(move || {
            sender.send("hello world!");
            t.unpark();
        });
        while !receiver.is_ready() {
            thread::park();
        }
        assert_eq!(receiver.receive(), "hello world!");
    });
}
