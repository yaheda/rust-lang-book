#![allow(unreachable_code, unused_labels)]

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

}
