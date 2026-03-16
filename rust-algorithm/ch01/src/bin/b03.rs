use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    assert!(n >= 3);

    let exists = a
        .iter()
        .any(|&ai| a.iter().any(|&aj| a.iter().any(|&ak| ai + aj + ak == 1000)));

    if exists {
        println!("Yes");
    } else {
        println!("No");
    }
}
