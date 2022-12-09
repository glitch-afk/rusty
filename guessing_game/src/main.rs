use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("🤫 Secret Number : {secret_number}");

    println!("Please input a guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("⚠️ Failed to read line.");

    let guess: u32 = guess.trim().parse().expect("⚠️ Please enter a number");

    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("Too Big"),
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("You win! 🎉"),
    }
}
