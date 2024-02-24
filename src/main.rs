use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is a guessing game!");
    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut guess: String = String::new();
    loop {
        guess.clear();
        println!("Please guess a number: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
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
