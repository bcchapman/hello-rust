fn main() {
    run_section("Vectors", vectors);
    run_section("Strings", strings);
    run_section("Hash Maps", hash_maps);
}

fn run_section(title: &str, section_fn: fn()) {
    println!("---------------------------");
    println!("{}", title);
    println!("---------------------------");
    section_fn();
}

fn vectors() {
    // specify type if not inferrable
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    // you can access elements by index
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // or using get
    let third = &v.get(2);
    if let Some(third) = third {
        println!("The third element is {}", third);
    }

    // alternatively you can use the vec! macro
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // you can also create a vector with a specific capacity
    let v: Vec<i32> = Vec::with_capacity(10);
    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("The first element is {}", first);
    // cannot modify the vector while borrowing it immutably
    // v.push(6), // this would cause a compile-time error
    // because vectors store data in contiguous memory and may need to move

    // iteratoring over vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // mutable iterator over vector
    // this allows you to modify the elements in place
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // use deference operator to modify the value
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }

    // vectors only store data of the same type
    // we can use enums to store different types

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        match i {
            SpreadsheetCell::Int(value) => println!("Integer: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }

    {
        let v = vec![1, 2, 3, 4];
        // v is valid here
        println!("Vector v: {:?}", v);
        // do stuff with v
    } // <- v goes out of scope and is freed here
}

fn strings() {
    // Time for Strings
    // starting empty
    let _ = String::new();

    // alternatively, you can create a string from a string literal
    let data = "initial contents";
    data.to_string();

    // The method also works on a literal directly:
    let _ = "initial contents".to_string();

    // or using the String::from function
    let _ = String::from("initial contents");

    // strings are UTF-8 encoded, so we can include any properly encoded data in them
    let hellos = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("_"),
        String::from("שלום"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    for hello in hellos {
        println!("{}", hello);
    }

    // appending to string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s); // prints "foobar"

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s); // prints "lol"

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3); // prints "Hello, world!"

    // concatenating strings is messy with multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s); // prints "tic-tac-toe"

    // alternatively, you can use the format! macro
    // need to redefine s1 to avoid moving it
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s); // prints "tic-tac-toe"

    // referencing characters by index in a string is not valid in Rust
    let s1 = String::from("hi");
    println!("s1 is {}", s1);
    // below does not compile
    //let h = s1[0];

    // you can use slices, however if you use an invalid size it will panic at runtime
    let hello = "Здравствуйте";
    let _ = &hello[0..4]; //valid
    // let _ = &hello[0..1]; // not-valid

    // iterating over characters in a string
    let hello = String::from("Hola");
    for c in hello.chars() {
        println!("{c}");
    }

    // iterating over bytes in a string
    for b in "Hola".bytes() {
        println!("{b}");
    }
}

use std::collections::HashMap;
fn hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for {}: {}", team_name, score);

    // can also iterate over contents
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // For types that implement the Copy trait, like i32, the values are copied into the hash map. 
    // For owned values like String, the values will be moved and the hash map will be the owner of those values,
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("Field name: {}", field_name);
    // println!("Field value: {}", field_value);   

    // overwritting values
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // entry allows you to check for an existing value, or insert a default
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // update based on previous content
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}