// Rng is called a "trait". When trait is "in scope"
// The method gen_range is put in scope by this "trait".
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // The = makes us inclusive of upper bound.
    // Lower bound is inclusive by default.
    let secret_number = rand::thread_rng().gen_range(1..=3);

    println!("Please input your guess: ");

    // Simply returns a new instance of string
    // UTF-8 encoded, growable string
    let mut guess = String::new();
    // Note that :: indicates an "associated" function.
    // An associated function is a function implemented on
    // a type--how analogous is this to class methods?

    // Notice the passing of guess by ref--mut flag means
    // we can change the value there. THIS IS NOT A POINTER.
    // This is called "borrowing".
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Type conversion -- "SHADOWING", notice let is still used
    let guess: u32 = guess.trim().parse().expect("Number plz");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
