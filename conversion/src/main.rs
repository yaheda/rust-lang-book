use std::convert::From;
use std::convert::Into;

use std::convert::TryFrom;
use std::convert::TryInto;

use std::str::FromStr;
use std::num::ParseIntError;

use std::fmt;

fn main() {
    let my_str = "Hello, world!";
    let my_string = String::from(my_str);
    println!("My string: {}", my_string);

    let number = Number::from(42);
    println!("My number: {:?}", number);

    let int = 100;
    let number_from_into: Number = int.into();
    println!("My number from Into: {:?}", number_from_into);

    assert_eq!(EvenNumber::try_from(4), Ok(EvenNumber(4)));
    assert_eq!(EvenNumber::try_from(5), Err(String::from("5 is not an even number")));

    let result: Result<EvenNumber, String> = 6.try_into();
    assert_eq!(result, Ok(EvenNumber(6)));
    let result: Result<EvenNumber, String> = 7.try_into();
    assert_eq!(result, Err(String::from("7 is not an even number")));

    let circle = circle { radius: 5.0 };
    println!("{}", circle);

    let parsed: i32 = "42".parse().expect("Not a number!");
    println!("Parsed number: {}", parsed);
    let parsed: i32 = "5".parse().unwrap(); // unwrap will panic if the parse fails, but since "5" is a valid number, it will succeed. However, it's generally safer to use expect or handle the error instead of unwrap in production code.
    let turbo_parsed: i32 = "5".parse::<i32>().unwrap();
    println!("Parsed number: {}", parsed);
    println!("Turbo parsed number: {}", turbo_parsed);

    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum);

    let radius = "   3";
    let circle: circle = radius.trim().parse().expect("Not a valid radius!");
    println!("Parsed circle: {}", circle);
}

struct circle {
    radius: f64,
}
impl fmt::Display for circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle with radius: {}", self.radius)
    }
}
impl FromStr for circle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f64>() {
            Ok(radius) => Ok(circle { radius }),
            Err(_) => Err(format!("Could not parse '{}' as a circle", s)),
        }
    }
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// from and into are intechangeable, so we can implement Into for Number as well, but it's not necessary since From is already implemented. 
// The Into trait is automatically implemented for types that implement From. 
// So we can use the Into trait without explicitly implementing it for Number.

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(format!("{} is not an even number", value))
        }
    }
}
