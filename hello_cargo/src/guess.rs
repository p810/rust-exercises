use std::cmp::Ordering;
use std::result::Result;

trait GuessResponse {
    fn getMessage(self) -> &'static str;
}

impl GuessResponse for IncorrectGuess {
    fn getMessage(self) -> &'static str {
        return match self.reason {
            Reason::TooLow => "Try going a little lower",
            Reason::TooHigh => "Try going up a bit",
        }
    }
}

impl GuessResponse for CorrectGuess {
    fn getMessage(self) -> &'static str {
        return "Congrats, you guessed correctly!";
    }
}

pub struct CorrectGuess {}

pub struct IncorrectGuess {
    pub reason: Reason,
}

pub enum Reason {
    TooLow,
    TooHigh,
}

pub enum Guess {
    Correct,
    Incorrect,
}

pub fn get_result(input: u32, number: u32) -> Result<GuessResponse, GuessResponse> {
    return match input.cmp(&number) {
        Ordering::Equal => Ok(CorrectGuess {}),
        Ordering::Less => Err(IncorrectGuess {
            reason: Reason::TooLow,
        }),
        Ordering::Greater => Err(IncorrectGuess {
            reason: Reason::TooHigh,
        }),
    };
}
