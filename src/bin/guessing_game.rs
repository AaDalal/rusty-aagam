use std::{io::stdin, cmp::Ordering};
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess a number!!");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read your line!");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(s) => panic!("Your guess isn't a number: {s}"),
        };

        // check the guess
        match guess.cmp(&secret) {
            Ordering::Less => println!("A little higher..."),
            Ordering::Greater => println!("...A little lower"),
            Ordering::Equal => {
                println!("JUST right.");
                break;
            }
        }
    }

}