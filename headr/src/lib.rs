use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
type MyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_index, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_index > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    // std::io::Read::takeは、最大リミットバイトを読み取りReadの新しいインスタンスを返す
                    // fn take(self, limit: u64) -> Take<Self>
                    // as を使って usize から u64 へキャストしている
                    let mut handle = file.take(num_bytes as u64);
                    // 0で初期化したnum_bytes長の可変なバッファーの作成
                    let mut buffer = vec![0; num_bytes];
                    // 実際に読み込まれたバイトを文字列に変換
                    // 有効なUTF-8でない場合もある
                    let bytes_read = handle.read(&mut buffer)?;

                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]))

                    // // より安全な方法
                    // let bytes: Result<Vec<_>, _> = file.bytes().take(num_bytes).collect();
                    // print!("{}", String::from_utf8_lossy(&bytes?));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        };
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Mizuki <mizu.mizupica@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .short('n')
                .long("lines")
                .conflicts_with("bytes")
                .help("print the first NUM lines instead of the first 10;\n\twith the leading '-', print all but the last\n\tNUM lines of each file")
                .value_parser(parse_positive_int)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .short('c')
                .long("bytes")
                .conflicts_with("lines")
                .help("print the first NUM bytes of each file;\n\twith the leading '-', print al but the last\n\tNUM bytes of each file")
                .value_parser(parse_positive_int)
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .cloned()
            .collect(),
        lines: matches
            .get_one::<usize>("lines")
            // 参照を実際の値に変換している
            .copied()
            // Optionから値を取り出す
            .unwrap_or(10),
        bytes: matches.get_one::<usize>("bytes").copied(),
    })
}

// 文字列を数値に変換するためのメソッド
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

// 入力ファイル処理ようのメソッド
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3は生の整数なのでOK
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // 数字でない文字列の場合はエラー
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0の場合もエラー
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
