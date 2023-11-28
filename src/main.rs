use rand::prelude::thread_rng;
use rand::prelude::Rng;
use std::io;

const MIN_NUMBER: i32 = 0;
const MAX_NUMBER: i32 = 10;

fn main() {
    let mut guess_count = 0;
    let mut guess;
    let mut rng = thread_rng();
    let correct_number = rng.gen_range(MIN_NUMBER..MAX_NUMBER);

    println!("Welcome to an amazing guessing game!");
    println!("Pick a number between {MIN_NUMBER} and {MAX_NUMBER}");

    guess = read_input();

    let mut guess_number = covert_to_int(guess);
    guess_count += 1;

    while guess_number != correct_number {
        guess_count += 1;

        println!("Incorrect! Guess again: ");

        guess = read_input();
        guess_number = covert_to_int(guess);
    }

    println!("Correct! You tried {guess_count} time(s)");
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    return input;
}

fn covert_to_int(input: String) -> i32 {
    input
        .trim_start()
        .trim_end()
        .to_string()
        .parse::<i32>()
        .unwrap()
}
