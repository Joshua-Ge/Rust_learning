use rand::prelude::*;

fn main() {
    let mut rng = rand::rng();
    let mut word = String::new();
    for _ in 0..10{
        let random_word = rng.sample(rand::distr::Alphanumeric) as char;
        word.push(random_word);
    }

    println!("Password: {}", word);

}
