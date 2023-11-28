use rand::prelude::thread_rng;
use rand::prelude::Rng;
use std::io;

const MIN_NUMBER: i32 = 0;
const MAX_NUMBER: i32 = 10;

fn main() {
    let mut guess_count = 0;
    let mut rng = thread_rng();
    let correct_number = rng.gen_range(MIN_NUMBER..MAX_NUMBER);

    println!("Welcome to an amazing guessing game!");
    println!("Pick a number between {MIN_NUMBER} and {MAX_NUMBER}");

    let mut guess_number = read_number_input();

    guess_count += 1;

    while guess_number != correct_number {
        guess_count += 1;

        println!("Incorrect! Guess again: ");

        guess_number = read_number_input();
    }

    println!("Correct! You tried {guess_count} time(s)");
}

fn read_number_input() -> i32 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: i32 = match input.trim().parse::<i32>() {
            Ok(parsed_input) => parsed_input,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        return input;
    }
}
