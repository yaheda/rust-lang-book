#![allow(dead_code)]

fn main() {
    println!("Hello, world!");

    let name = String::from("Alice");
    let age = 30;
    let alice = Person { name, age };
    println!("Alice is {:?}", alice);

    // Instantiate a point
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point = Point { x: 10.3, y: 0.2 };
    println!("point coordinates: ({}, {})", point.x, point.y);
    // make a new point by using struct update syntax to use the fields of our other point
    let bottom_right = Point { x: 10.3, ..another_point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Deconstructure the point using a let binding
    let Point { x: left_edge, y: top_edge } = point;
    println!("left edge: {}, top edge: {}", left_edge, top_edge);

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Deconstructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("integer: {}, decimal: {}", integer, decimal);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit; // unit struct

struct Pair(i32, f32); // tuple struct

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
