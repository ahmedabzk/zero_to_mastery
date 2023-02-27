fn main() {
    let value = 90;
    let bool_value = value > 100;

    print_message(bool_value);
}

fn print_message(a: bool) {

    match a{
        true => println!("its big"),
        false => println!("its small"),
    }
}