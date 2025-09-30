use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// 構造体の定義
// コマンドライン引数を管理するのに使う
// deriveマクロでは、Debugトレイトを追加して構造体を表示できるようにする
#[derive(Debug)]
// Configという名前のパブリックな構造体を定義
pub struct Config {
    // ファイル名を格納するための文字列ベクタ
    files: Vec<String>,
    // 行番号を表示するかどうかを示す真理値
    number_lines: bool,
    // 空行以外に行番号を表示するかどうかを示す真理値
    number_nonblank_lines: bool,
    // 連続した空白行を1行にまとめるかどうかを示す真理値
    squeeze_blank_lines: bool,
}

// ジェネリックを用いて任意の型に対するOKを返せるように型エイリアスを作成
type MyResult<T> = Result<T, Box<dyn Error>>;

// パブリック関数の定義
// デフォルトではモジュール内の全ての変数と関数はプライベート
pub fn run(config: Config) -> MyResult<()> {
    let mut cur_row: u32 = 1;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut previous_line_was_blank = false;

                // file.lines : 読み込んだファイルの行のいてレーターを返す
                for line in file.lines() {
                    let line = line?;

                    // -s のロジック
                    if config.squeeze_blank_lines {
                        match (previous_line_was_blank, line.is_empty()) {
                            (true, true) => continue,
                            (false, true) => previous_line_was_blank = true,
                            (_, _) => previous_line_was_blank = false,
                        }
                    }

                    // -nと-bのロジック
                    if (config.number_nonblank_lines && !line.is_empty()) || config.number_lines {
                        print!("{:>6}\t", cur_row);
                        cur_row += 1;
                    };
                    println!("{}", line);
                }
            }
        }
    }
    Ok(())
}

// ユーザーが実行時に与えた値を使って、新しいConfigを作成する
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number lines")
                .conflicts_with("number_nonblank")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number nonblank lines")
                .conflicts_with("number_lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("squeeze_blank_lines")
                .short('s')
                .long("squeeze-blank")
                .help("supress repeated empty output lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .expect("file is required")
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
        squeeze_blank_lines: matches.get_flag("squeeze_blank_lines"),
    })
}

// ファイル名を受け取り、エラーかBufRadトレイトを実装したBox型の値を含むOkを返す
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        // ファイル名が-の場合、std::io::stdinから読み込む
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        // それ以外の場合、File::openを用いて、与えられたファイルを開こうとする
        // エラーの場合はそれを伝搬させる
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
