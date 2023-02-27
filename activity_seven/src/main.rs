enum Color{
    Red,
    Green,
    Black,
    White,
    Blue,
}

impl Color{
    fn print(&self) {
        match self{
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Black => println!("black"),
            Color::White => println!("white"),
            Color::Blue => println!("blue"),
        };
    }
}

struct Dimension {
    width: i32,
    height: i32,
    depth: i32,
}

impl Dimension {
    fn print(&self) {
        println!("{}", self.width);
        println!("{}", self.height);
        println!("{}", self.depth);
    }
}

struct Shippingbox {
    dimension: Dimension,
    weight: i32,
    color: Color,

}

impl Shippingbox {
    fn new(weight: i32, color: Color, dimension: Dimension) -> Self {
        Self { 
            dimension,
            weight, 
            color
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimension.print();
        println!("{}", self.weight);
    }
}

fn main() {

    let small_box = Dimension{
        width: 10,
        height: 5,
        depth: 3,
    };

    let newbox = Shippingbox::new(15, Color::Black, small_box);

    newbox.print();


}


