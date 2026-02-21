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

    let area = rect_area(_rectangle);
    println!("area of rectangle: {}", area);

    let square_point = Point { x: 0.0, y: 0.0 };
    let square_rect = square(square_point, 5.0);
    println!("{:?}", square_rect);
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { top_left, bottom_right } = rectangle;
    let width = bottom_right.x - top_left.x;
    let height = top_left.y - bottom_right.y;
    width * height
}

fn square(point: Point, size: f32) -> Rectangle {
    let Point { x, y } = point;
    let rectangle = Rectangle {
        top_left: point,
        bottom_right: Point {
            x: x + size,
            y: y - size
        }
    };
    rectangle
}

// Add a function square_new which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32
// fn square_new(point: Point, size: f32) -> Rectangle {
//     Rectangle {
//         top_left: point,
//         bottom_right: Point {
//             x: point.x + size,
//             y: point.y - size
//         }
//     }
// }

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit; // unit struct

struct Pair(i32, f32); // tuple struct

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
