#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square (size: u32) -> Rectangle{
        Rectangle {
            width: size,
            height:size,
        }
    }
}
fn main() {
    println!("Hello, structs!");

    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    if rect1.width(){
        println!(" the rectangle has a nonzero width; it is {}", rect1.width);
    }
    println!("can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    let sq =Rectangle::square(3);
    println!(" The square area: {:?}", sq);
}
