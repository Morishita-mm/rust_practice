#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    println!("Hello, world!");
    let green = Color::Green;
    println!("Green Color Code: {}", color_to_str(&green));
    find_maybe_number(Some(4));
    find_maybe_number(None);

    let maybe_number = Some(6);
    // let maybe_number:Option<i32>  = None;
    if let Some(number) = maybe_number {
        println!("number: {}", number);
    } else {
        println!("this is else stmt");
    }
}

fn find_maybe_number(maybe_number: Option<u32>) {
    match maybe_number {
        Some(number) => println!("found {}", number),
        None => println!("Nothing found."),
    }
}

fn color_to_str(color: &Color) -> &str {
    // Red #FF0000
    // Green #00FF00
    // Blue #0000FF
    match color {
        Color::Red => "#FF0000",
        Color::Green => "#00FF00",
        Color::Blue => "#0000FF",
    }
}
