use crate::game::*;
use crate::reader::get_file_contents;
use std::io;

mod clean;
mod game;
mod reader;

fn main() {
    let file_contents = get_file_contents();
    let random_word = get_random_word(file_contents);
    let mut game = Game::new(random_word);

    loop {
        println!("{}", game.get_shown());
        println!("What is your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.len() > 1 {
            println!("Enter a single alphabetical character");
            continue;
        }

        let guess = guess.chars().next().unwrap();
        if !guess.is_ascii_alphabetic() {
            println!("Enter a single alphabetical character");
            continue;
        }

        if game.try_letter(guess) {
            println!("Correct");
        } else {
            println!("Incorrect");
        }

        if !game.get_shown().contains("_") {
            println!("Congratz you won in {} attempts", game.get_attempts());
            return;
        }
    }
}
