use std::fmt;

fn main() {
    println!("hello world");

    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );
    println!("Base 10 {}", 69420); // default base is 10
    println!("Base 2: {:b}", 69420); // binary
    println!("Base 8: {:o}", 69420); // octal
    println!("Base 16: {:x}", 69420); // hexadecimal

    println!("{number:>5}", number=1); // right-aligned with 5 spaces
    println!("{number:0>5}", number=1); // right-aligned with 5 zeros
    println!("{number:0<5}", number=1); // left-aligned with 5 zeros
    println!("{number:>width$}", number=1, width=6);

    //println!("My name is {0}, {1} {0}", "Bond");

    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}", pi=pi);
    println!("Hello {0} is {1:.5}", "x", 0.01);
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);
    
    println!("Hello {} is {:.*}", "x", 5, 0.01);
    /* 
        - Mechanism: The "Argument Pointer" starts at 0.
        - {} takes index 0 ("x"). Pointer moves to 1.
        - .* takes index 1 (5) for precision. Pointer moves to 2.
        - The remaining part of {:.*} takes index 2 (0.01).
    */

    println!("Hello {1} is {2:.*}", 5, "x", 0.01);
    /*
        - Mechanism: When you use a number like {1} or {2}, you are jumping around the argument list, but the .* still follows the "Next Available" rule.
        - {1} explicitly jumps to index 1 ("x").
        - {2:.*} explicitly jumps to index 2 (0.01) for the value.
        - The Trap: The .* inside that tag says: "Give me the argument immediately following the one we just used."
        - Since the last-used index was 2, the .* looks for index 3.
     */


    println!("Hello {} is {2:.*}", "x", 5, 0.01);
    /*
        Looks like the .* is the next index from {}
     */

    println!("Hello {} is {number:.prec$}", "x", number=0.01, prec=5);

    println!("{}, `{name:.*}` has 3 fractinal digits", "Hello", 3, name=1234.56);
    println!("{}, `{name:.*}` has 3 characters", "Hello", 3, name="1234.56");
    println!("{}, `{name:>8.*}` is 3 right-aligned characters", "Hello", 3, name="1234.56");

    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Alice", "Bob", actor="main character");
    println!("{:?} will print", Structure(42));
    println!("{:?} will print", Deep(Structure(42)));

    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{:#?}", peter);

    println!("Structure1(42) will print `{}`", Structure1(42));

    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let bigrange = MinMax(-300, 300);
    let smallrange = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}",
        big=bigrange,
        small=smallrange);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let complex = Complex { real: 4.7, imag: -2.3 };
    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Structure1(i32);
impl fmt::Display for Structure1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ben zona {}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:+}i", self.real, self.imag)
    }
}


