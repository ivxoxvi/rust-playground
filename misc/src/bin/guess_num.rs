use std::{cmp::Ordering, io};

use rand::RngExt;

fn main() {
    println!("Guess the number!");
    const ADMIN_PWD: u32 = 999;
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed: {guess}");

        if guess == ADMIN_PWD {
            println!("I salute you, my lord");
            return;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("success");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }
}