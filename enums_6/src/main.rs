#[derive(Debug)] // so we can inspect the state in a minute
enum UsState{
    Alabama,
    Alaska,
    //--snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cets(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("State quarter from {:?}!", state);
            25
        }

    }
}
fn main(){
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    fn add_fancy_hat(){}
    fn remove_fancy_hat(){}
    fn move_player(num_spaces: u8){}

    fn plus_one(x: Option<i32>) -> Option<i32>{
        match x{
            None => None, 
            Some(i) => Some (i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// #![allow(unused)]
// fn main(){
//     enum Option<T>{
//         None,
//         Some(T),
//     }
//     let some_number = Some(5);
//     let some_string = Some("a string");

//     let absent_number: Option<i32> = None;
// }



// enum Message{
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32,i32,i32)
// }

// impl Message{
//     fn call(&self){
//        //method body would be defined here 
//     }
// }

// fn main(){
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }