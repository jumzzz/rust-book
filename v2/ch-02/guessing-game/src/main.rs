use std::io::stdin;
use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Input your guess:");
    
    let right_guess: u16 = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("Failed to read line.");
    let guess_parsed: u16 = guess[0..guess.len() - 1]
                        .parse()
                        .unwrap_or(0);

    println!("You guessed: {}", guess_parsed);
    println!("You guessed right: {} == {}, {}",right_guess, guess_parsed, right_guess == guess_parsed);
}
