use std::num::NonZeroU128;

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    
    let students = vec![
        Student{
            name: "Hassan".to_owned(),
            locker: Some(12),
        },
        Student{
            name: "Ahmed".to_owned(),
            locker: Some(5),
        },
        Student{
            name: "Yasir".to_owned(),
            locker: None,
        },

    ];

    for student in students{
        println!("{}", student.name);
        match student.locker {
            Some(num) => println!("locker number: {}", num),
            None => println!("No")
        }
    }
}
