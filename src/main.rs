use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is a guessing game!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);
    let mut guess: String = String::new();
    loop {
        guess.clear();
        println!("Please guess a number from 1 to 10 (or enter Q to quit): ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().to_lowercase() == "q" {
            println!("You quit the game!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
