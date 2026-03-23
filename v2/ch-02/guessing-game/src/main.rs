use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    const MIN_RANGE: u16 = 1;
    const MAX_RANGE: u16 = 100;
    let secret_number = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);
    
    loop {
        println!("Guess the number: {MIN_RANGE}-{MAX_RANGE}");
        println!("Input your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("{guess} is too small!"),
            Ordering::Greater   => println!("{guess} is too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            } 
        }
    }
}
