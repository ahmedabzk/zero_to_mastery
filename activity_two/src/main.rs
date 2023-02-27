
#[allow(unused)]
enum Color {
    Red,
    Yellow,
    Blue,
}

fn main() {
    getting_color(Color::Blue);
}

fn getting_color(my_color: Color) {
    match my_color {
        Color::Red => println!("red"),
        Color::Yellow => println!("yellow"),
        Color::Blue => println!("Blue"),
    }
}
