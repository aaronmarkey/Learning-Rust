use std::cmp::Ordering; // Ordering is an enum of Less, Greater, Equal.
use std::io; // Import io from stdlib.

// A "trait" rng generators implement. Must be in scope, discussed in Ch10.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Create rng gen for thread, then generate immut num between 1 and 100.
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret}");

    loop {
        println!("Please input your guess:");

        // Declare a mutable variable, UTF8 string.
        let mut guess = String::new();

        // From stdin, read input.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadow guess as immutable, trim, and parse into u32 type.
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n, // When Ok, simply return the value.
            Err(_) => {
                // _ is a catch-all for all possible error types.
                println!("Not a number, try again.");
                continue; // Start loop over again.
            }
        }; // Statement is still ending, thus semicolon.

        // String interpolation.
        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            // `match` is a switch on a pattern.
            // Each pattern is called an "arm".
            Ordering::Less => println!("Too small."), // Single line anon func.
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                // Multiline anon function.
                println!("Correct");
                break; // Break the loop.
            }
        }
    }
}
