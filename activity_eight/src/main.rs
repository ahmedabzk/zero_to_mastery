fn main() {
    let my_numbers = vec![10, 20, 30, 40];

    
    for num in &my_numbers{
        match num {
            30 => println!("thirty"),
            _ => println!("{}", num),
        }
    }

    println!("the length of the vector = {}",my_numbers.len());

    let mut newvec = Vec::new();
    newvec.push(5);
    newvec.push(10);
    newvec.push(8);
    newvec.pop();

    newvec.len();



    
}
