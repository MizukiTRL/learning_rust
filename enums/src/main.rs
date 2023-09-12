fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hola gato"));
    m.call();

    let nums: [i32; 10] = Default::default();

    for i in nums{
        println!("{i}");
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let seis = six.unwrap();

    println!("{}", six.unwrap());

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

}

enum IpAddrKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message  {
    fn call(&self){
       match self {
           Self::Write(value) => println!("{value}"),
           Self::ChangeColor(x, y, z) => println!("{}", x),
           Self::Move { x, y } => println!("moved"),
           Self::Quit => println!("exited"),
       }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    texas,
    florida,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

