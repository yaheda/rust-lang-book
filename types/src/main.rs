// suppress all errors from casts which overflow
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;
    
    // Error! No implicit conversion
    //let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting to an unsigned type, T, std::T::MAX + 1 is added or subtracted until the value fits into T
    // fits into new type ONLY when the ![allow(overflowing_literals)] attribute is used
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept, while the rest towards the most significant bit (MSB) get truncated.
    // explain further: 1000 in binary is 1111101000, the 8 least significant bits are 11101000, which is 232 in decimal
    println!("1000 as a u8 is: {}", 1000 as u8);

    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // for positive numbers, this is the same as the modulus operator
    println!("1000 mod 256 is: {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as first casting to the corresponding unsigned type. 
    // If the most significant bit of that value is 1, then the value is negative.

    println!("128 as a i16 is: {}", 128 as i16);
    println!("128 as a i8 is: {}", 128 as i8);

    // repeating the example above
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("1000 as a i8 is: {}", 1000 as i8); // 1000 as u8 is 232, which has the most significant bit set to 1, so it is negative when interpreted as i8. The value is -24 because 232 - 256 = -24.

    // For floating-point to integer casts, the number is rounded towards zero (truncated). If the number is too large to fit in the target type, the behavior is undefined.
    println!("300.0 as a u8 is: {}", 300.0_f32 as u8); // 255 
    println!("-300.0 as a u8 is: {}", (-300.0_f32) as u8); // 0
    println!("nan as u8 is: {}", f32::NAN as u8); // NaN is treated as 0 when cast to an integer

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        println!("300.0 as a u8 is: {}", 300.0_f32.to_int_unchecked::<u8>()); // 300 truncated to fit into u8 results in 44 (300 - 256 = 44)
        println!("-300.0 as a u8 is: {}", (-300.0_f32).to_int_unchecked::<u8>()); // -300 truncated to fit into u8 results in 212 (-300 + 256 = -44, and then -44 + 256 = 212)
        println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>()); // NaN is treated as 0 when cast to an integer
    }

    println!("********** Literals");

    let x = 1u8; // 1 with the type u8
    let y = 2u32; // 2 with the type u32
    let z = 3f32; // 3.0 with the type f32

    let i = 1;
    let f = 1.0;

    println!("size of x in bytes: {}", std::mem::size_of_val(&x));
    println!("size of y in bytes: {}", std::mem::size_of_val(&y));
    println!("size of z in bytes: {}", std::mem::size_of_val(&z));
    println!("size of i in bytes: {}", std::mem::size_of_val(&i));
    println!("size of f in bytes: {}", std::mem::size_of_val(&f));

    println!("********** Inference");

    let elem = 5u8; // elem has type u8
    let mut vec = Vec::new(); // vec has type Vec<???>

    vec.push(elem); // vec has type Vec<u8>

    println!("vec: {:?}", vec);

    println!("********** Aliasing");

    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64;
    let u: U64 = 123456789;

    println!("nanoseconds: {}", nanoseconds);
    println!("inches: {}", inches);
    println!("u: {}", u);

    println!("{} nanoseconds + {} inches = {} units?", nanoseconds, inches, nanoseconds + inches);

}

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;