use std::io;
use rand::Rng;

fn main() {
    println!("> Guess the number I am thinking of! <");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //debug line
    println!("The secret number is {secret_number}");

    println!("Please input the guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
