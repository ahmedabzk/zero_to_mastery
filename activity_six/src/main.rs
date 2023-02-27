struct Grocery{
    quantity: i32,
    id: i32,
}

fn main() {
    let grocery = Grocery{
        quantity: 4,
        id: 123,
    };

    display_quantity(&grocery);
    display_id(&grocery);
}

fn display_quantity(item: &Grocery) {
    println!("quantity = {}", item.quantity);
}

fn display_id(item: &Grocery){
    println!("id number = {}", item.id);
}
