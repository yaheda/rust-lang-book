fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 10;

    println!("Is {} divisible by {}? {}", x, y, is_divible_by(x, y));

    println!("******* Methods and Associated Functions *******");
    let p1 = Point::origin();
    println!("The origin is at ({}, {})", p1.x, p1.y);
    let p2 = Point::new(3.0, 4.0);
    println!("The point is at ({}, {})", p2.x, p2.y); 
    let rect = Rectangle { width: 3.0, height: 4.0 };
    println!("The area of the rectangle is {}", rect.area());

    println!("******* Closures *******");
    let add = |a: i32, b: i32| a + b;
    println!("The sum of 5 and 10 is {}", add(5, 10));

    let outer_var = 20;
    let closed_anonoted = |i: i32| -> i32 { i + outer_var };
    println!("The result of the closure is {}", closed_anonoted(5));

    let closed_inferred = |i: i32| i + outer_var;
    println!("The result of the closure is {}", closed_inferred(10));

    let one = || 1;
    println!("The result of the closure is {}", one());

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
