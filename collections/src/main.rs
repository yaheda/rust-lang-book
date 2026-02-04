use std::collections::HashMap;

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

    // -------------------------------
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for team {}: {}", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //println!("{}", field_name); // this line would cause a compile-time error because field_name has been moved into the map

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // this will overwrite the previous value
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // this will not overwrite the existing value
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // get a mutable reference to the value
        *count += 1;
    }
    println!("{:?}", map);

    exercise1();
    exercise2();
    exercise3();
}

fn exercise1() {
    let v = vec![1, 2, 3,1, 4, 5];
    let median_value = if v.len() % 2 == 0 {
        let mid = v.len() / 2;
        format!("Median value: {} and {}", (v[mid - 1]), v[mid])
    } else {
        format!("Median value: {}", v[v.len() / 2])
    };
    println!("{}", median_value);
}

fn exercise2() {
    let voyels = "aeiou";
    let consonants = "bcdfghjklmnpqrstvwxyz";

    for c in voyels.chars() {
        println!("{} is a voyel", c);
    }
    for c in consonants.chars() {
        println!("{} is a consonant", c);
    }

    let word = "first";
    let ay = "ay";

    let mut pig_latin = String::new();
    let first_char = word.chars().next().unwrap(); // get the first char. Unwrap is safe here because word is not empty
    if voyels.contains(first_char) {
        pig_latin = format!("{}-{}", word, "hay");
        println!("{}", pig_latin);
    } else {
        let rest_of_word: String = word.chars().skip(1).collect(); // collect rest of the chars after the first one. collect into a String
        pig_latin = format!("{}-{}{}", rest_of_word, first_char, ay);
        println!("{}", pig_latin);
    }
}
use std::io;

fn exercise3() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    let mut console_input = String::new();
    // io::stdin().read_line(&mut console_input).expect("Failed to read input");
    // let input_parts: Vec<&str> = console_input.trim().split_whitespace().collect();
    // println!("{:?}", input_parts);
    // let employee_name = input_parts[1].to_string();
    // let department_name = input_parts[3].to_string();
    // departments.entry(department_name).or_insert(Vec::new()).push(employee_name);
    // println!("{:?}", departments);

    while console_input.trim() != "exit" {
        console_input.clear();
        io::stdin().read_line(&mut console_input).expect("Failed to read input");
        let input_parts: Vec<&str> = console_input.trim().split_whitespace().collect();
        if input_parts.len() == 4 && input_parts[0] == "Add" && input_parts[2] == "to" {
            let employee_name = input_parts[1].to_string();
            let department_name = input_parts[3].to_string();
            departments.entry(department_name).or_insert(Vec::new()).push(employee_name);
            println!("{:?}", departments);
        } else {
            println!("Invalid input. Please use the format: 'Add <employee_name> to <department_name>' or type 'exit' to quit.");
        }
    }
    // for word in console_input.split_whitespace() {
    //     println!("{}: {}", word, word.len());
    // }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
