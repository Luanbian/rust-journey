use std::io::{self};
use rand::Rng;
use std::cmp::Ordering;

pub fn main() {
    let secret_number = generate_secret_number();
    let user_guess = get_user_guess();
    let result = compare_guess_to_secret_number(user_guess, secret_number);
    println!("{}", result);
}

fn get_user_guess() -> u32 {
    println!("Guess the number!");
    println!("Please, input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            get_user_guess()
        }
    };

    return guess;
}

fn generate_secret_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    return secret_number;
}

fn compare_guess_to_secret_number(guess: u32, secret_number: u32) -> String {
    match guess.cmp(&secret_number) {
        Ordering::Less =>  "Too small!".to_string(),
        Ordering::Greater => "Too big!".to_string(),
        Ordering::Equal => "You win!".to_string(),
    }
}
