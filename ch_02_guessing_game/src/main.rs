use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    println!("secret number is {secret_number}");

    println!("Guess the number");

    loop {
        let mut guess: String = String::new();

        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
