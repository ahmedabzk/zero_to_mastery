fn main() {
   let (x, y) = returns_tuple();

    if y > 5{
        println!("greater than 5");
    }else if y < 5{
        println!("less than 5")
    }else{
        println!("y is equal to 5")
    }
}

fn returns_tuple() -> (i32, i32){
    (1, 5)
}
