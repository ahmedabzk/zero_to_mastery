fn main() {
    println!("Hello, world!");
    let fname = String::from("Ahmed");
    println!("first name = {}", first_name(fname));
    let lname = String::from("Hassan");
    println!("last name = {}", last_name(lname));
}

// first name
fn first_name(fname: String) -> String{
    fname
}

// last name
fn last_name(lname: String) -> String{
    lname
}