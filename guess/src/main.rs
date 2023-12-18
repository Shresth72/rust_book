use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret: i32 = rand::thread_rng().gen_range(0..100);

    println!("The secret number is : {}", secret);

    println!("Please input your guess.");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess) // append to ref to the guess string
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
