use std::collections::HashMap;

// struct Furniture {
//     name: String,
// }
fn main() {
    let mut furnitures = HashMap::new();

    let mut total = 0;

    furnitures.insert("Chairs", 5);
    furnitures.insert("Beds", 3);
    furnitures.insert("Tables", 2);
    furnitures.insert("Couches", 0);

    for (key, value) in furnitures {
        total += value;
        if value == 0 {
            println!("out of stock");
        } else {
            println!("name = {}, number of items = {}", key, value);
        }
    }

    println!("total number of items = {}", total);
}
