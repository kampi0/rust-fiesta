#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}
fn main() {
    println!("Hello, structs!");

    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    println!("
    The are of rectangle is {} square pixels", 
rect1.area()
);
}

    // let scale = 2;
    // let rect2 = Rectangle{
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    // dbg!(&rect2);

//     println!(
//         "the area of rectangle is {} square pixels.", 
//         area(&rect1)
        
//     );
//     println!("rect1 is {:#?}", rect1);
// }
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
//     }
//     ################################################
    // let mut user1 = User {
    //     email: String::from("someone@examle.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // user1.email = String::from("anotheremail@example.com");
    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);
// }

//  fn build_user(email: String, username: String) -> User {
//      User {
//          email,
//          username,
//          active: true,
//          sign_in_count: 1,
//      }
//  }