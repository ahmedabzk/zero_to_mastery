#[derive(Debug)]

enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGeust;

impl Priority for ImportantGeust {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]

struct Guest;

impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

fn print_out<T>(prio: T)
where
    T: Priority +  std::fmt::Debug
{
    // let priority = prio.get_priority();
    println!("{:?} is {:?} priority", prio, prio.get_priority());
}

fn main() {

    let guest = Guest;
    let vip = ImportantGeust;

    print_out(guest);
    print_out(vip);
}
