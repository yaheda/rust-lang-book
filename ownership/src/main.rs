fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = String::from("hello");
    takes_ownership(s3);
    //println!("s3 = {}", s3);

    let x1 = 5;
    makes_copy(x1);

    let s4 = gives_ownership();
    let s5 = String::from("hello");
    let s6 = takes_and_gives_back(s5);

    let mut s7 = String::from("Ben Zona");
    let word = first_word(&s7);
    println!("{}", word);
    s7.clear(); 
    //println!("{}", word);

    let s8 = String::from("Shar Mutta");
    let s8_1 = &s8[0..4];
    let s8_2 = &s8[5..10];
    println!("{}", s8_1); println!("{}", s8_2);
    let slice1 = &s8[0..4];
    let slice2 = &s8[..4];

    let a = [1,2,3,4,5];
    let slice3 = &a[..2];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
