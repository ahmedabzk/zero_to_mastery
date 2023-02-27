#[allow(unused)]
enum Flavors{
    Orange,
    Apple,
    Mango,
    Lemon,
}

struct Drink{
    flavor: Flavors,
    fluid_oz: f64,
}
fn main() {
    let orange = Drink{
        flavor: Flavors::Orange,
        fluid_oz: 6.1,
    };

    print_drink(orange);
}

fn print_drink(drink: Drink){
    match drink.flavor{
        Flavors::Orange => println!("orange"),
        Flavors::Apple => println!("Apple"),
        Flavors::Mango => println!("Mango"),
        Flavors::Lemon => println!("Lemon"),
    }

    println!("fluid_oz: {:?}", drink.fluid_oz);
    
}
