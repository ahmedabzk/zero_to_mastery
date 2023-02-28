enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, "Hassan".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(35.0, String::from("Ahmed")),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => println!("backstage ticket {}, {}", holder, price),
            Ticket::Standard(price) => println!("standard ticket {}", price),
            Ticket::Vip(price, holder) => println!("vip ticket {}, {}", holder, price),
        }
    }
}
