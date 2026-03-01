#![allow(unreachable_code, unused_labels, dead_code)]

use std::array;

fn main() {
    println!("********** if/else");

    let n = 5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!("{} has only one digit", n);
            1
        } else {
            println!("{} has multiple digits", n);
            2
        }; // do not forget the semicolon here, otherwise big_n will be of type () and the value will be () as well, which is not what we want.
    println!("The value of big_n is: {}", big_n);

    println!("********** loop");

    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue; // skip the rest of the loop and start the next iteration
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break; // exit the loop
        }
    }

    println!("********** nested loops and labels");

    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer; // break the outer loop from the inner loop
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    println!("********** returning from loops");

    let mut count = 0u32;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // return count * 2 from the loop
        }
    };
    println!("The result is {}", result);

    println!("********** while");

    let mut n = 1;
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    println!("********** for");
    // same as above but with a for loop and a range
    for n in 1..10 { // or for n in 1..=9 to include 9
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() { // This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.
        match name {
            &"Ferris" => println!("There is a rustacean among us!"), // without the & here, we would be matching against the value of name, which is a reference to a string slice, and we would not be able to match against the string literal "Ferris" because it is of type &str, not &str. By using &"Ferris", we are matching against the value that name is referencing, which is a string slice, and we can match against the string literal "Ferris" because it is of type &str.
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names); // names is still available here because we iterated over references to the elements of the vector, not the elements themselves. If we had iterated over the elements themselves, then names would have been moved and we would not be able to use it after the loop.

    for name in names.into_iter() { // This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop
        match name {
            "Ferris" => println!("There is a rustacean among us!"), // without the & here, we would be matching against the value of name, which is a string slice, and we would be able to match against the string literal "Ferris" because it is of type &str. By using "Ferris", we are matching against the value of name directly, which is a string slice, and we can match against the string literal "Ferris" because it is of type &str.
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names); // names is no longer available here because it has

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() { // This mutably borrows each element of the collection, allowing for the collection to be modified in place.
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!", // without the &mut here, we would be matching against the value of name, which is a mutable reference to a string slice, and we would not be able to match against the string literal "Ferris" because it is of type &str, not &str. By using &mut "Ferris", we are matching against the value that name is referencing, which is a mutable reference to a string slice, and we can match against the string literal "Ferris" because it is of type &str.
            _ => "Hello",
        };
    }
    println!("names: {:?}", names); // names mutatted in place, so we can see the changes here.

    println!("********** match");

    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} converts to {}", boolean, binary);

    println!("********** deconstructuring tuples");
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is 0, y is {} and z is {}", y, z),
        (1, ..) => println!("First is 1 and the rest doesn't matter"),
        (.., 2) => println!("Last is 2 and the rest doesn't matter"),
        (3, .., 4) => println!("First is 3, last is 4 and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    println!("********** deconstructuring arrays and slices");

    let array = [1, -2, 6];
    println!("Tell me about {:?}", array);
    match array {
        [0, second, third] => println!("First is 0, second is {} and third is {}", second, third),
        [1, _, _] => println!("First is 1 and the rest doesn't matter"),
        [1, _, third] => println!("First is 1, third is {} and the rest doesn't matter", third),
        [-1, second, ..] => println!("First is -1, second is {} and the rest doesn't matter", second),
        [3, second, tail @ ..] => println!("First is 3, second is {} and the rest is {:?}", second, tail),
        [first, middle @ .., last] => println!("First is {}, middle is {:?} and last is {}", first, middle, last),
        _ => println!("It doesn't matter what they are"),
    }

    println!("********** deconstructuring enums");

    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),
        Color::RGB(r, g, b) => println!("The color is RGB({}, {}, {})", r, g, b),
        Color::HSV(h, s, v) => println!("The color is HSV({}, {}, {})", h, s, v),
        Color::HSL(h, s, l) => println!("The color is HSL({}, {}, {})", h, s, l),
        Color::CMY(c, m, y) => println!("The color is CMY({}, {}, {})", c, m, y),
        Color::CMYK(c, m, y, k) => println!("The color is CMYK({}, {}, {}, {})", c, m, y, k),
    }

    println!("********** deconstructuring pointers and references in patterns");

    let reference = &4;

    match reference {
        &value => println!("Got a value via destructuring: {}", value),
    }

    // to avoid the extra & in the pattern, we can dereference the reference in the match expression itself:
    match *reference {
        value => println!("Got a value via dereferencing: {}", value),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3; // this is the same as let _is_a_reference = &3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10; // we can modify the value through the mutable reference
            println!("Got a mutable reference to a value: {}", m);
        },
    }

    println!("**********  deconstructuring struts");

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // try changing the values in the struct to see what happens.
    let foo = Foo { x: (3, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b is {} and y is {}", b, y),

        // renamed fields
        Foo { x: (a, 2), y: c } => println!("First of x is {}, second is 2 and y is {}", a, c),

        // ignore some fields
        Foo { y, .. } => println!("y is {} and the rest doesn't matter", y),
    }

    // you do not need match to destructure a struct, you can use let bindings as well.
    let Foo { x: (a, b), y } = foo;
    println!("a is {}, b is {} and y is {}", a, b, y);

    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: Foo { x: (1, 2), y: 3 } };
    let Bar { 
        foo: Foo { 
            x: (a, b), 
            y 
        } 
    } = bar;
    println!("a is {}, b is {} and y is {}", a, b, y);

    println!("**********  match guards");

    let temperature = Temperature::Celsius(35.0);
    match temperature {
        Temperature::Celsius(t) if t > 30.0 => println!("It's hot!"),
        Temperature::Celsius(t) if t < 10.0 => println!("It's cold!"),
        Temperature::Celsius(t) => println!("It's {} degrees Celsius", t),
        Temperature::Fahrenheit(t) => println!("It's {} degrees Fahrenheit", t),
    }

    let number = 4u8;
    match number {
        i if i == 0 => println!("zero"),
        i if i > 0 => println!("greater than zero"),
        _ => println!("less than zero"), // this case will never be reached because u8 cannot be negative, but we need to include it to make the match expression exhaustive.
    }

    println!("********** match @ bindings");

    println!("Tell me about the person you are");
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {}", n),
        n @ 13..=19 => println!("I'm a teen of age {}", n),
        n @ (1 | 7 | 15 | 13) => println!("I'm a teen of age {}", n),
        n => println!("I'm an old person of age {}", n),
        
    }

    match some_number() {
        Some(n @ 42) => println!("The answer to the Ultimate Question of Life, The Universe, and Everything is {}", n),
        Some(n) => println!("Just a regular number: {}", n),
        _ => (),
    }

    println!("********** if let");

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    
    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`). 
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("Didn't match a number. Let's go with an emoticon!");
    }

    let a = Foo1::Bar;
    let b = Foo1::Baz;
    let c = Foo1::Qux(100);

    if let Foo1::Bar = a {
        println!("a is foobar");
    }
    if let Foo1::Baz = b {
        println!("b is foobaz");
    }
    if let Foo1::Qux(n) = c {
        println!("c is fooqux with value {}", n);
    }

    println!("********** let-else");

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    println!("********** while let");

    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("i is {:?}, try again", i);
                    optional = Some(i + 1);
                }
            },
            _ => break,
        }
    }

    optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("W Greater than 9, quit!");
            optional = None;
        } else {
            println!("W i is {:?}, try again", i);
            optional = Some(i + 1);
        }
    }


}

use std::str::FromStr;
fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Input string is not in the correct format");
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("Count is not a valid number");
    };
    (count, item)
}

enum Foo1 {
    Bar,
    Baz,
    Qux(u32),
}

fn some_number() -> Option<u32> {
    Some(7)
}

fn age() -> u32 {
    15
}

enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

enum Color {
    Red,
    Green,
    Blue,

    RGB(u8, u8, u8), // tuple struct variant
    HSV(u8, u8, u8), // tuple struct variant
    HSL(u8, u8, u8), // tuple struct variant
    CMY(u8, u8, u8), // tuple struct variant
    CMYK(u8, u8, u8, u8), // tuple struct variant
}

