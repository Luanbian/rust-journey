use std::io::{self};
use rand::Rng;


pub fn main() {
    generate_secret_number();
    get_user_guess();
}

fn get_user_guess() -> String {
    println!("Guess the number!");
    println!("Please, input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    return guess;
}

fn generate_secret_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    return secret_number;
}
