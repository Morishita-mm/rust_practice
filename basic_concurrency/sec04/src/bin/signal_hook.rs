use signal_hook::{iterator::Signals, consts::SIGUSR1}; // ❶
use std::{error::Error, process, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    // プロセスIDを表示
    println!("pid: {}", process::id());

    let mut signals = Signals::new(&[SIGUSR1])?; // ❷
    thread::spawn(move || {
        // シグナルを受信
        for sig in signals.forever() { // ❸
            println!("received signal: {:?}", sig);
        }
    });

    // 10秒スリープ
    thread::sleep(Duration::from_secs(10));
    Ok(())
}