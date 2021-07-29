use rand::Rng;
use std::io::{Write, stdout, stdin};
use substring::Substring;

fn main() {
    let mut string: String = String::new();
    let mut rng = rand::thread_rng();
    loop {
        string.clear();
        print!("Enter your guess (1-10): ");
        if stdout().flush().is_err() {
            panic!("Broken pipe?!");
        }
        if stdin().read_line(&mut string).is_err() {
            panic!("Broken pipe?!");
        }
        string = String::from(string.substring(0, string.len() - 1));
        let guess: i16 = string.parse().unwrap_or(-1);
        if guess > 0 && guess <= 10 {
            let number: i16 = rng.gen_range(1..10);
            if guess == number {
                println!("You win!");
            } else {
                println!("You lose. The correct number was: {}", number);
            }
            break;
        } else {
            println!("Invalid entry!");
        }
    }
}
