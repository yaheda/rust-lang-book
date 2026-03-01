fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 10;

    println!("Is {} divisible by {}? {}", x, y, is_divible_by(x, y));

    println!("******* Methods and Associated Functions *******");


}

struct Point {
    x: f64,
    y: f64,
}
impl Point {
    // this is an associated function, it doesn't take self as a parameter
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    // this is a method, it takes self as a parameter
    fn area(&self) -> f64 {
        self.width * self.height
    }
}



fn is_divible_by(lhs: i32, rhs: i32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}
