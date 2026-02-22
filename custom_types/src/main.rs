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

    println!("****************** Enums");
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned()); // creates an owned 'String' from a string literal
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let op = Operations::Add;
    let result = op.run(1, 2);
    println!("result: {}", result);
    op.benzona();

    // Explicitly 'use'
    use Stage::{ Beginner, Advanced };
    use Role::*;

    let stage = Beginner; // equivalent to Stage::Beginner
    let role = Student; // equivalent to Role::Student

    match stage {
        Beginner => println!("Beginner"),
        Advanced => println!("Advanced"),
    }

    match role {
        Student => println!("Student"),
        Teacher => println!("Teacher"), 
    }

    // cast as integers
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as u32);
    println!("violets are #{:06x}", Color::Blue as u32);

    println!("****************** Testcase linked list");
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.strigify());

    let list2 = List::Cons(10, Box::new(List::Cons(20, Box::new(List::Nil))));
    println!("{}", list2.strigify());

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", 3, is_big(3));
}

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

use crate::List::*;

#[derive(Debug)]
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}
impl List {
    // Create an empty list
    fn new() -> List {
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn strigify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.strigify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Substract,
}
// create a type of alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
    fn benzona(&self) {
        println!("Sharmuta");
    }
}


enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64}
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaded"),
        WebEvent::PageUnload => println!("Page Unloaded"),
        WebEvent::KeyPress(c) => println!("Key pressed: {}", c),
        WebEvent::Paste(s) => println!("Pasted something: {}", s),
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}", x, y)
    }
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
