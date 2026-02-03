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
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
