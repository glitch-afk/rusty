use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}", "Guessing Game!".on_bright_cyan());

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // uncomment the line below to see the generated secret number
    // println!("🤫 Secret Number : {secret_number}");

    loop {
        println!("{}", "Please input a guess.".blue());

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("⚠️ Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "⚠️ Please enter a number".red());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("{}", "You win! 🎉".green());
                break;
            }
        }
    }
}
