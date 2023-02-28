struct Customer {
    age: i32,
}

fn restricted_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age < 21 {
        Err("Customer must be at least 21 years old".to_owned())
    }else{
        Ok(())
    }
}


fn main() {
    let hassan = Customer{age: 22};

    let purchase = restricted_purchase(&hassan);

    println!("{:?}", purchase);

}
