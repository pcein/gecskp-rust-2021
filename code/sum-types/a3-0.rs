enum Color {
    Red,
    Green,
    Blue,
}

fn print_name(c: Color) {
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

fn main() {
    let c = Color::Green;
    print_name(c);
}
