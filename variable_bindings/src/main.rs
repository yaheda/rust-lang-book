fn main() {
    let an_integer = 1u32;
    let _a_boolean = true;
    let _unit = ();

    let copied_integer = an_integer;

    println!("An integer: {}, copied integer: {}", an_integer, copied_integer);

    let _used_variable = 3u32;
    let _noisy_unused_variable = 2u32;

    println!("******************* Mutable and immutable variables");
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    println!("******************* Shadowing");
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }
    println!("outer long: {}", long_lived_binding);

    let shadowed_binding = 1;
    {
        println!("Before shadowing: {}", shadowed_binding);
        let shadowed_binding = "abc";
        println!("After shadowing: {}", shadowed_binding);
    }
    println!("Outside shadowing: {}", shadowed_binding);
    let shadowed_binding = 2;
    println!("Shadowed again: {}", shadowed_binding);

    println!("******************* Decalre first, then assign");
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let another_binding;
    another_binding = 1;
    println!("another binding: {}", another_binding);

    println!("******************* Freezing");
    let mut mutable_integer = 1;
    {
        let _mutable_integer = mutable_integer;
        // mutable_integer = 2; // error: cannot assign to `mutable_integer` because it is borrowed
    }
    mutable_integer = 2; // this is fine because the immutable borrow of `mutable_integer` has ended
    println!("mutable integer: {}", mutable_integer);


    

}
