use assert_cmd::Command;

#[test]
fn works() {
    // コマンドの実行が成功したか
    let mut cmd = Command::new("pwd");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello_cargo").unwrap();
    // コマンドの実行が成功したか
    cmd.assert().success();
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    // コマンドの実行が成功したか
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    // コマンドの実行が失敗したか
    cmd.assert().failure();
}

#[test]
fn main() {
    let mut cmd = Command::cargo_bin("hello_cargo").unwrap();
    // 指定したコマンドにおける標準出力と実際の出力が等しいか
    cmd.assert().success().stdout("Hello, world!\n");
}