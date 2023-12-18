// OWNERSHIP MODEL
/*
    1. Each value in Rust has a variable that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
 */

fn main() {
    { // s is not valid here, it's not displayed here.
        let _s: &str = "hello"; 
        let _s: String = String::from("hello"); // stored on heap
        // do stuff with s
    } // this scope is over now, and s is no longer valid



    // In Stack
    let x: i32 = 5;
    let _y: i32 = x; // Copy

    // In Heap
    let s1: String = String::from("World");
    let s2: String = s1; // Move (not shallow copy, ie, s1 is invalidated nd can't be used)

    let _s3: String = s2.clone(); // Copy



    

    // Ownership in functions
    let s = String::from("helloWorld");
    takes_ownership(s);
    // println!("{}", s); // Move

    let taker_string:String = gives_ownership();
    println!("{}", taker_string); // owner_string Moved to taker_string





    // References to use variables without giving up ownership
    let s1 = String::from("hello");
    println!("{}", s1);

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);



    // Mutable References (only one mutable reference to a particular piece of data in a particular scope)
    let mut s = String::from("hello");
    change(&mut s);

    

    // Dangling References (references to memory that has been deallocated)
    // let reference_to_nothing = dangle();





    // Slices (reference to a contiguous sequence of elements in a collection rather than the whole collection)
    let s = String::from("hello world");
    let _hello: &str = &s[0..5];
    let _world: &str = &s[6..11];

    let _hello_world: &str = &s[..];
    let _hello_world: &str = &s[..11];
    let _hello_world: &str = &s[0..];
    let _hello_world: &str = &s[..=10];


    let s = String::from("hello world");
    let word: &str = first_word(&s);
    println!("{}", word);


    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() { // enumerate returns a tuple (index, reference)
        if item == b' ' { // b' ' is byte literal
            return &s[0..i];
        }
    }
    &s[..]
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope, and is dropped. Its memory goes away.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn takes_ownership(string: String) {
    println!("{}", string);
}

fn gives_ownership() -> String {
    let owner_string = String::from("uwu");
    owner_string
}
