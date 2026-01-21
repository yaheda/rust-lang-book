fn main() {
    let mut user1 = User {
        username: String::from("benzona"),
        active: true,
        email: String::from("benzona@gmail.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.username);
    user1.email = String::from("benzona@yahoo.com");
    println!("{}", user1.email);

    let user2 = build_user(String::from("shar mutta"), String::from("sharmutta@gmail.com"));
    println!("{}", user2.username);

    let user3 = User {
        username: String::from("zabi"),
        ..user2
    };
    println!("{}", user2.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Color(r, g, b) = black;
    println!("{}", r);
    println!("{}", g);
    println!("{}", b);

    let width = 10;
    let height = 20;
    let area = area(width, height);
    println!("The area of the rectangle is {} square pixels.", area);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(10 * scale),
        height: 30,
    };
    let area = area1(&rect1);
    println!("The area of the rectangle with area1 is {} square pixels.", area);
    println!("The width of the rectangle is {} pixels.", rect1.width);

    println!("rect is {:#?}", rect1);
    dbg!(&rect1);

    let radius = 30.0;
    let area = area_of_circle(radius);
    println!("The area of the circle is {} square pixels.", area);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of rect2 is {} square pixels",
        rect2.area()
    );
    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(30);
    println!("The area of square is {} square pixels", square.area());
}

fn area_of_circle(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

struct User {
    username: String,
    active: bool,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(username: String, email: String) -> User {
    User {
        username,
        active: true,
        email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
