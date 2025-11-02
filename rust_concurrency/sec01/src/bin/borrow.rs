fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        x();    // never reached
    }
}

fn main() {
    f(& 21, &mut 50);
}

fn x() {
    panic!("Something wrong");
}