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

    value_in_cents(Coin::Quarter(USState::Alaska));

    let five = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(Option::None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        num @ 1..=6 => move_player(num),
        _ => reroll(),
        //_ => (),
    }

    let config_max = Option::Some(3u8);
    match config_max {
        Option::Some(max) => println!("The max is configured to be {}", max),
        _ => (),
    }

    let config_max2 = Option::Some(3u8);
    if let Option::Some(max) = config_max2 {
        println!("The max is configured to be {}", max);
    }

    let coin = Coin::Quarter(USState::Alabama);
    let mut count = 0;
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count2 = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count2 += 1;
    }

    let v1 = describe_state_in_quarter(Coin::Quarter(USState::Alabama));
    println!("{:?}", v1);
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}



fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1)
    }
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}

impl USState {
    fn existed_in_year(&self, year: i32) -> bool {
        match self {
            USState::Alabama => year >= 1819,
            USState::Alaska => year >= 1859,
        }
    }
}

fn describe_state_in_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in_year(1900) {
            Option::Some(format!("{:?} existed in 1900", state))
        } else {
            Option::Some(format!("{:?} did not exist in 1900", state))
        }
    } else {
        Option::None
    }
}

fn describe_state_in_quarter2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return Option::None;
    };

    if state.existed_in_year(1900) {
        Option::Some(format!("{:?} existed in 1900", state))
    } else {
        Option::Some(format!("{:?} did not exist in 1900", state))
    }
}

fn describe_state_in_quarter3(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return Option::None;
    };

    if state.existed_in_year(1900) {
        Option::Some(format!("{:?} existed in 1900", state))
    } else {
        Option::Some(format!("{:?} did not exist in 1900", state))
    }
}




enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>  {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
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

#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}
