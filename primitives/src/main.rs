fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32; // suffixes make literals be of a certain type
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    let mut inferred_type = 12; // Type i64 is inferred from the next line
    inferred_type = 4294967296i64;

    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    let mutable = true; // Mutable shadowing

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let my_tuple = (5u32, 1u8, true, -5.04f32); // Tuple with a bunch of different types

    println!("1_000_000 is {}", 1_000_000);

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("1e64 is {}, -2.5e-3 is {}", 1e64, -2.5e-3);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 OR 0101 is {:04b}", 0b0011 | 0b0101);
    println!("0011 and 0101 is {:04b}", 0b0011 & 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is {}", 0x80u32 >> 2);
}
