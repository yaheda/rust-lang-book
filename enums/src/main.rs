fn main() {
    println!("Hello, world!");

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    let home2 = IpAddrKind::V4(String::from("127.0.0.1"));

    let home3 = IpAddr2Kind::V4(127,0,0,1);
    let loopback3 = IpAddr2Kind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_numer = Option::Some(5);
    let some_string = Some("string");
    let absent_number: Option<i32> = Option::None;
}

enum IpAddrKind {
    V4(String),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

enum IpAddr2Kind {
    V4(u8,u8,u8,u8),
    V6(String)
} 

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Message received! {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color: {}, {}, {}", r, g, b),
        }
    }
}

enum Option<T> {
    None,
    Some(T),
}
