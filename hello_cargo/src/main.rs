use rand::{thread_rng, Rng};
use std::io::stdin;
mod guess;

fn main() {
    let number = get_secret_number();

    println!("I'm thinking of a number between 1 and 10. Please enter your guess.");

    loop {
        let mut input = String::new();

        stdin().read_line(&mut input)
            .expect("Failed to read user input");
        
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a number as your guess");
                continue
            },
        };

        let result = guess::get_result(input, number);

        let output = match guess::get_result(input, number) {
            Ok(r) => r.getMessage(),
            Err(guess::IncorrectGuess) => {
                match result.reason {
                    Some(guess::Reason::TooLow) => "Try going a little lower.",
                    Some(guess::Reason::TooHigh) => "Try going a little higher.",
                    None => "We shouldn't be here...", // to do: find a better way to implement *all* of this
                }
            },
        };
    
        println!("{}", output);

        if input == number {
            break;
        }
    }
}

fn get_secret_number() -> u32 {
    return thread_rng().gen_range(1, 11);
}
