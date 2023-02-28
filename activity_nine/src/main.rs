struct Info {
    name: String,
    color: String,
    age: i32,
}

fn print_info(name: &str, color: &str){
    println!("{}, {}", name, color);
}

fn main() {
    let persons = vec![
        Info{
            name: String::from("Hassan"),
            color: "Yellow".to_owned(),
            age: 9,
        },
        Info{
            name: String::from("Ahmed"),
            color: "Red".to_owned(),
            age: 10,
        },
        Info{
            name: String::from("Yeahyeah"),
            color: "Black".to_owned(),
            age: 15,
        },
    ];

    for person in persons{
        if person.age <= 10{
            print_info(&person.name, &person.color);
        }
    }
}
