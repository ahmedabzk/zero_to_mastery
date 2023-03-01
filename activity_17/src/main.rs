use std::{io};

enum States{
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}


impl States {
    fn from_str(s: &str) -> Option<States>{
        let s = s.trim().to_lowercase();
        match s.as_str(){
            "off" => Some(States::Off),
            "sleep" => Some(States::Sleep),
            "reboot" => Some(States::Reboot),
            "shutdown" => Some(States::Shutdown),
            "hibernate" => Some(States::Hibernate),
            _ => None,
        }
    }
}

fn power_messages(msg: States) {
    match msg{
        States::Off => println!("going off"),
        States::Sleep => println!("going to sleep mode"),
        States::Reboot => println!("rebooting"),
        States::Shutdown => println!("shutting down"),
        States::Hibernate => println!("hibernating"),
    }
}
fn main() {
    let mut input = String::new();

    let user_input = io::stdin().read_line(&mut input);

    if user_input.is_ok() {
        match States::from_str(&input) {
            Some(state) => power_messages(state),
            None => println!("invalid power state"),
        }
    }else{
        println!("error reading input");
    }


}
