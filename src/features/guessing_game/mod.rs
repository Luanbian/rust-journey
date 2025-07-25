use rand::Rng;
use std::cmp::Ordering;
use std::io::{self};

pub fn main() {
    println!("Guess the number!");
    let secret_number = generate_secret_number();

    loop {
        println!("The secret number is between 1 and 100: {}", secret_number);
        let user_guess = get_user_guess();
        let result = compare_guess_to_secret_number(user_guess, secret_number);

        if result == "You win!" {
            println!("Congratulations! You guessed the secret number!");
            break;
        } else {
            println!("{} Try again!", result);
        }
    }
}

fn get_user_guess() -> u32 {
    println!("Please, input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            get_user_guess()
        }
    };

    guess
}

fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn compare_guess_to_secret_number(guess: u32, secret_number: u32) -> String {
    match guess.cmp(&secret_number) {
        Ordering::Less => "Too small!".to_string(),
        Ordering::Greater => "Too big!".to_string(),
        Ordering::Equal => "You win!".to_string(),
    }
}
