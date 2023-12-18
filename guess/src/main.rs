use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret: i32 = rand::thread_rng().gen_range(0..100);

    println!("The secret number is : {}", secret);

    loop {
        println!("Please input your guess.");
    
        let mut guess: String = String::new();
    
        io::stdin()
            .read_line(&mut guess) // append to ref to the guess string
            .expect("Failed to read line");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "The input must be a number".red());
                continue;
            },
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
}
