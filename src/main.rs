use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!");

    let number = rand::thread_rng().gen_range(1..=100);
    println!("{number}"); // only for test

    print!("Insert your guess: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error in guess input");

    println!("Your guess: {guess}");
}