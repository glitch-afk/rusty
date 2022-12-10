use colored::*;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", "SHA1 CRACKER".yellow());

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print!("{}", "Usage:".on_yellow());
        println!(" {}", "sha1_cracker: <wordlist.txt> <sha1_hash>".blue());
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?.trim().to_string();
        println!("{}", line);
    }

    Ok(())
}
