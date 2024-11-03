use std::io::{self};

pub fn main() {
    get_user_guess();
}

fn get_user_guess() {
    println!("Guess the number!");
    println!("Please, input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
