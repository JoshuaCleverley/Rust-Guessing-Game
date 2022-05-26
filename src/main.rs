/*
    Tutorial from chapter 2 of the rust programming language book
    https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html


    Joshua Cleverley
*/

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number game in rust");
    
    let secret = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please enter your guess (1-100):");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = 
            match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number");
                    continue;
                },
            };
        
        println!("Your guess was {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            },
        }
    }
}
