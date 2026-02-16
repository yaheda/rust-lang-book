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