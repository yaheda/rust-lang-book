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

    // tuples
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 3), (4u64, -1i8, -2), (-3i32, -4i64, 0.1f32));
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // but long tuples cannot be printed (more than 12 elements)
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33);
    // println!("too long tuple: {:?}", too_longle_tuple);

    let pair = (42, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    // to create one element tuples, the comma is required to disambiguate from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("not a tuple: {:?}", (5u32));

    // tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix: \n{}", matrix);
    println!("transposed matrix: \n{}", transpose(matrix));

    // arrays and slices
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500]; // array of 500 integers initialized to 0
    println!("first element of the array: {}", xs[0]);
    println!("number of elements in the array: {}", xs.len());
    println!("array occupies {} bytes", std::mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the index of the first element in the slice
    // ending_index is the index of the first element after the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&xs[1..4]);

    // example of empty slice &[]
    let an_empty_slice: &[i32] = &[];
    println!("an empty slice: {:?}", an_empty_slice);
    assert_eq!(an_empty_slice.len(), 0);
    assert_eq!(an_empty_slice, &[]); // empty slice is equal to an empty array
    assert_eq!(an_empty_slice, &[][..]); // same as above

    // arrays can be safely accessed using .get, which returns an Option<&T>
    // This is because accessing an array with an index that is out of bounds will cause a panic
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(x) => println!("xs[{}] = {}", i, x),
            None => println!("xs[{}] is out of bounds", i),
        }
    }
}

use std::mem;
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
    println!("the slice occupies {} bytes", mem::size_of_val(slice));
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_part, bool_part) = pair;
    (bool_part, int_part)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}
