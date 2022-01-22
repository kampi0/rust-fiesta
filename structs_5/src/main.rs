struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}



fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("someone@examle.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
