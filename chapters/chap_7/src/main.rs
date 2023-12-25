use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Vectors
    let _a = [1, 2, 3, 4, 5];

    // let v: Vec<i32> = a.iter().map(|x| x * 2).collect();
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    v.push(6);
    v.push(9);
    println!("{:?}", v);

    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);







    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    // v.push(6); 
    println!("{}", third);

    match v.get(20) {
        Some(third) => println!("{}", third),
        None => println!("There is no third element."),
    }







    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }








    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    // for (i, item) in row.iter().enumerate() {
    //     match &row[i] {
    //         SpreadsheetCell::Int(i) => println!("{}", i),
    //         _ => println!("Not an int"),
    //     }
    // }

    for i in &row {
        match i {
            SpreadsheetCell::Int(item) => println!("This is an interger: {}", item),
            SpreadsheetCell::Float(item) => println!("This is a float: {}", item),
            _ => println!("Not an int"),
        }
    }










    // Strings
    // Strings are implemented as a Vec<u8> but guaranteed to always be valid UTF-8.

    let s1 = String::new();
    let s2 = "haha";
    let s3 = s2.to_string();
    let s4 = String::from("hihi");
    
    

    
    let mut s = String::from("foo");
    s.push_str("fighters");
    s.push('!');
    println!("{}", s);

    let s3 = s1 + &s; // s1 has been moved here and can no longer be used

    let s5 = format!("{} {} {}", s2, s3, s4);
    println!("{}", s5);



    let hello = String::from("hello");
    let s = &hello[2..3];
    println!("{}", s);

    for c in "सृजन".chars() {
        println!("{}", c);
    }

    for b in "नम".bytes() {
        println!("{}", b);
    }

    for g in "सृजन".graphemes(true) {
        println!("{}", g);
    }









    // Hash Maps
    use std::collections::HashMap;

    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50); // blue and yellow are invalid at this point

    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }



    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 25);

    let _s: &mut i32 = scores.entry(String::from("yellow")).or_insert(30);
    scores.entry(String::from("yellow")).or_insert(50); // does not overwrite
    println!("{:?}", scores);







    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    

}
