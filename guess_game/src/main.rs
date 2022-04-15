use std::io;
use std::cmp::Ordering;
use std::process::exit;
use rand::Rng;

fn main() {
    println!("Welcome to Guess Game!");
    println!("Enter your Guess ");

    let sec_code = rand::thread_rng().gen_range(1..100);
    
    // println!("sec_code = {}",sec_code);

    let mut guess = String::new();
    // let mut guess = "Your guess was ".to_string(); // can't assign "string" directly

    io::stdin()
    .read_line(&mut guess) // read_line() appends the user input to existing string without modifying it!!
    .expect("Failed to read the line");

    // let guess: u32 = guess.trim().parse().expect("Enter a number you Idiot");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => exit(-1),
    };

    match guess.cmp(&sec_code) {
        Ordering::Equal => println!("You won!!"),
        Ordering::Greater => println!("You lost guess was very high!"),
        Ordering::Less => println!("You lost guess was very low!"),
    }

    println!("The secret code was: {}",sec_code);
    // println!("Your guess was: {}",guess); // o/p : Your guess was {whatever number you enter}
}
