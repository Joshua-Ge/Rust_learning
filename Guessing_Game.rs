use std::io;

fn main() {
    println!("Welcome to the guessing game!");
    println!("Please make a guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Couldn't Assign that sorry");
    
    println!("You guessed {guess}");
}
