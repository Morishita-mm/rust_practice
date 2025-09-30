fn main() {
    // catr::get_args()がOk(config)を返した場合、Result：:and _thenを使ってcatr::runにconfigを渡す
    if let Err(e) = catr::get_args().and_then(catr::run) {
        // eprintln!マクロを使って、エラーメッセージを標準エラーに出力
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
