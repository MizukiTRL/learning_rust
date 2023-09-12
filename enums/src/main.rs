fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hola gato"));
    m.call();
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
       println!("{:?}", self);
    }
}

