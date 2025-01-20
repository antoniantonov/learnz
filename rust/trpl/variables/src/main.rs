use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Testing integer overflow...");
    if cfg!(debug_assertions) {
        println!("Running in debug mode");
    } else {
        println!("Running in release mode");
    }

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let mut guess: u8 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    println!("You guessed: {guess}");
    guess = guess + 1;
    println!("The guess was modified to: {guess}");
}