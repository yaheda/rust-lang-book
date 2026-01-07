fn main() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");

    let y = 55;
    let y = y + 1;
    {
        let y = y * 3;
        println!("y in inner scope is {y}");
    }
    println!("y in outer scope is {y}");

    let spaces = "123";
    println!("{spaces}");
    let spaces = spaces.len();
    println!("{}", spaces);

    let guess: u32 = "42".parse().expect("not a number");
    println!("{}", guess);

    let _x = 2.0;
    let _y: f32 = 3.0;

    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, _y, _z) = _tup;
    println!("{x}");
    let _five_hundred = _tup.0;

    let _a = [1,2,3,4,5];
    let _b: [i32; 5] = [6,7,8,9,10];
    let _c = [3; 5];
    benzona();
    seh(123);

    let y1 = {
        let x = 3;
        x + 1
    };
    println!("expression: {y1}");

    let f = five();
    println!("function five: {f}");
    let plus = plus_one(1);
    println!("function plus_one: {plus}");

    control_flow();

    repeating();
    repeating_remaining();
    looploop();
}

fn control_flow() {
    let number = 5;
    if number > 8 {
        println!("zabi");
    } else {
        println!("benzona")
    }

    let _n1 = if true {5} else {6};
}

fn repeating() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter// * 2
        }
    };
    println!("Repeating: {result}");
}

fn repeating_remaining() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
}

fn looploop() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFtoFF");
}

fn benzona() {
    println!("sharmutaa");
}

fn seh(x: i32) {
    println!("{x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
