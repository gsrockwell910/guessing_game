use std::io; // io library to get user input

use rand::Rng; // Rand library to generate random numbers

// Entry point of the program
fn main() {
    // Macro to print a String to the screen
    println!("Guess the number!");

    // Generate a secret number from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // Mutable variable to store user input
    let mut guess = String::new();

    // Handle user input
    // & indicates a reference
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Prints String containing user input
    println!("You guessed: {guess}");
}