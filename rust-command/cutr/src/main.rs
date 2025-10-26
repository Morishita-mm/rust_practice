fn main() {
    if let Error(e) = curt::get_args().and_then
    (cutr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}