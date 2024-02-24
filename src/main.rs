use std::io;
fn main() {
    let mut guess: String = String::new();
    println!("Please guess a number.");
    println!("Enter your guess: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
