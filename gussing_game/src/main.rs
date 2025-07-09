use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!");

    // Generate a secret number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);

    // Loop until the user guesses correctly
    loop {
        println!("Please enter your guess:");

        // Create a new mutable string to store user input
        let mut guess = String::new();

        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Try to convert the input string to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 🎉");
                break;
            }
        }
    }
}