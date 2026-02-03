fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();
    let _v1 = vec![1, 2, 3];

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let fifth: Option<&i32> = v.get(4);
    match fifth {
        Option::Some(fifth) => println!("The fifth element is {}", fifth),
        Option::None => println!("There is no fifth element."),
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50; // dereference i to get the value
        println!("{}", i);
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in _row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }

    // -------------------------------
    
    let mut _s = String::new();
    let _data = "initial contents";
    let _s1 = _data.to_string();
    let _s2 = "initial contents".to_string();
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("Hello, ");
    let s2 = "world";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used, fn signature of + is fn add(self, s: &str) -> String
    println!("{}", s3);
    println!("s2 is still usable: {}", s2);
    //println!("{}", s1); // this line would cause a compile-time error

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    
    let _hello = String::from("hello");
    //let h = hello[0]; // this line would cause a compile-time error
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4]; // 1st 4 bytes represent the first 2 chars
    println!("{}", s);
    //let s = &hello[0..2]; // this line would cause a runtime error, because 1st 2 bytes do not represent a valid char
    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
