use std::io;

fn main() {
    println!("What year is it");

    let mut current_year:String = String::new();
    let mut birth_year:String = String::new();

    io::stdin()
        .read_line(&mut current_year)
        .expect("Not a valid year");

    println!("What is your birth year");

    io::stdin()
        .read_line(&mut birth_year)
        .expect("Not a valid year");

    let current_year:u32 = current_year
        .trim()
        .parse()
        .expect("Couldn't parse");

    let birth_year:u32 = birth_year
        .trim()
        .parse()
        .expect("Couldn't parse");

    let age:u32 = current_year - birth_year;
    
    println!("Your age is {}", age);

}
