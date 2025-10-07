use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // コマンドラインから引数を受け取る
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("APplocation error: {}", err);

        process::exit(1);
    }
}
