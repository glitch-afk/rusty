use std::io;

fn main() {
    println!("Guessing Game!");

    println!("Please input a guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");
}
