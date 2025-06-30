use std::io;

fn main() {
    println!("Welcome to calculator!");
    println!("please select an actio +,-,*,/");

    let mut action = String::new();
    
    io::stdin()
        .read_line(&mut action)
        .expect("action not supported");

    let action = action.trim();

    if action == "+" {
        add();
    } else if action == "-" {
        subtract();
    } else if action == "*" {
        multiply();
    } else if action == "/" {
        divide();
    } else {
        println!("Action is not supported");
    }

}

fn add() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("What is your first number?");

    io::stdin()
        .read_line(&mut num1)
        .expect("Not a valid number");

    println!("What is your second number");
    io::stdin()
        .read_line(&mut num2)
        .expect("Not a valid number");

    let number1 : i32 = num1 
        .trim()
        .parse()
        .expect("Not a valid number");

    let number2 : i32 = num2 
        .trim()
        .parse()
        .expect("Not a valid number");

    let sum = number1 + number2;

    println!("answer = {sum}");
}

fn multiply() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("What is your first number?");

    io::stdin()
        .read_line(&mut num1)
        .expect("Not a valid number");

    println!("What is your second number");
    io::stdin()
        .read_line(&mut num2)
        .expect("Not a valid number");

    let number1 : i32 = num1 
        .trim()
        .parse()
        .expect("Not a valid number");

    let number2 : i32 = num2 
        .trim()
        .parse()
        .expect("Not a valid number");

    let sum = number1 * number2;

    println!("answer = {sum}");
}

fn subtract() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("What is your first number?");

    io::stdin()
        .read_line(&mut num1)
        .expect("Not a valid number");

    println!("What is your second number");
    io::stdin()
        .read_line(&mut num2)
        .expect("Not a valid number");

    let number1 : i32 = num1 
        .trim()
        .parse()
        .expect("Not a valid number");

    let number2 : i32 = num2 
        .trim()
        .parse()
        .expect("Not a valid number");

    let sum = number1 - number2;

    println!("answer = {sum}");
}

fn divide() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("What is your first number?");

    io::stdin()
        .read_line(&mut num1)
        .expect("Not a valid number");

    println!("What is your second number");
    io::stdin()
        .read_line(&mut num2)
        .expect("Not a valid number");

    let number1 : f64 = num1 
        .trim()
        .parse()
        .expect("Not a valid number");

    let number2 : f64 = num2 
        .trim()
        .parse()
        .expect("Not a valid number");

    let sum:f64 = (number1 / number2).into();

    println!("answer = {sum}");
}
