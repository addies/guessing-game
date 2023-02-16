use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    //Generating random number from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    //Receiving input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //Converting String to Int
    let input: u32 = guess
        .trim()
        .parse()
        .expect("Wanted a number");    
    
    if secret_number == input {
        println!("You are the Best");
    } else {
        println!("You are wrong");
    }

    println!("You guessed: {guess}");

 }