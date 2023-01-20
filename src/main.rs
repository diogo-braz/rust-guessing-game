use std::cmp::Ordering;
use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!");

    let number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Insert your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error in guess input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Insert a number!");
                continue;
            }
        };

        println!("Your guess: {guess}");

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Your guess is correct!");
                break;
            }
        }
    }
}
