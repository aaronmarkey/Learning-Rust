use std::cmp::Ordering; // Ordering is an enum of Less, Greater, Equal.
use std::io; // Import io from stdlib.

// A "trait" rng generators implement. Must be in scope, discussed in Ch10.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Create rng gen for thread, then generate immut num between 1 and 100.
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret}");

    println!("Please input your guess:");

    // Declare a mutable variable, UTF8 string.
    let guess = String::new();

    // From stdin, read input.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // String interpolation.
    println!("You guessed: {guess}");

    match guess.cmp(&secret) {
        // `match` is a switch on a pattern.
        // Each pattern is called an "arm".
        Ordering::Less => println!("Too small."),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => "Correct",
    }
}
