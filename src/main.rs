use std::{io, cmp::Ordering};

use rand::Rng;

fn main() -> () {
    // - We're going to share to the user
    // "welcome to the game, please input a guess for the number"
    println!("Welcome to the game! Please provide an input for the guess!");
    // - get the user input
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
    .expect("Failed to process the user input");

    println!("We received this guess: {}", user_input);
    // Parse out our number
    let user_number: u32 = user_input.trim().parse()
    .unwrap();

    // - generate the random number
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=10);
    match user_number.cmp(&number) {
        Ordering::Less => {
            println!("You should guess a larger number!");
        },
        Ordering::Greater => {
            println!("You should guess a smaller number!");
        },
        Ordering::Equal => {
            println!("You guessed the right number! Woohoo!");
        }
    }
    // - evaluate if the number is correct
    //    1. Is it too high?
    //    2. Is it too low?
    //    3. Is it just right?

}
