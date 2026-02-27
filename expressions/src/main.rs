fn main() {
    // let x = 5;
    // x;
    // x + 1;
    // 15;
    // println!("The value of x is: {}", x);

    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;
        x_cubed + x_squared + x + 1
    };
    let z = {
        2 * x; // This expression evaluates to 10, but the value is not used, so it is discarded.
    };

    println!("The value of y is: {}", y);
    println!("The value of z is: {:?}", z); // z is of type (), which is the unit type, and it has only one value, which is also ().
}
