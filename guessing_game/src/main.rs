use rand::Rng;
use std::{cmp::Ordering, num::ParseIntError, io};

mod guessing_game;
use guessing_game::Guess;

fn main() {
    println!("Guess the number...");

    // reset guesses and create secret number
    let secret_number = initiatize_game();
    let mut guesses: u32 = 0; 

    loop {
        // ask for guess and validate int
        let guess = match prompt_guess() {
            Ok(guess) => Guess::new(guess),
            Err(_) => continue,
        };

        // increment guesses, input was valid
        guesses += 1;

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too small!"),
            Ordering::Greater => println!("Your guess was too big!"),
            Ordering::Equal => {
                println!("You win in {} guesses!", guesses);
                break;
            }
        }
    }
}

fn initiatize_game() -> i32 {
    return rand::rng().random_range(1..=100);
}

fn prompt_guess() -> Result<i32, ParseIntError>  {
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess.trim().parse()
}
