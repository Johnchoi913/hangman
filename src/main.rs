use crate::ai::*;
use crate::game::*;
use crate::reader::get_file_contents;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

mod ai;
mod clean;
mod game;
mod reader;

fn main() {
    let file_contents = Arc::new(get_file_contents());
    let ai = Arc::new(Mutex::new(Ai::new()));

    let mut handles = Vec::new();
    for _ in 0..4 {
        let ai = Arc::clone(&ai);
        let file_contents = Arc::clone(&file_contents);

        let handle = thread::spawn(move || {
            for _ in 0..25_000 {
                let random_word = get_random_word(&file_contents);
                let mut game = Game::new(random_word);

                while !game.get_finished() {
                    let mut ai = ai.lock().unwrap();
                    train(&mut game, &mut ai);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let random_word = get_random_word(&file_contents);
    let mut game = Game::new(random_word);
    let mut ai = ai.lock().unwrap();

    while !game.get_finished() {
        train(&mut game, &mut ai);
    }

    println!("{} attempts", game.get_attempts());
}

fn train(game: &mut Game, ai: &mut Ai) {
    let prev_shown_word = game.get_shown().to_string();
    let prev_used_letters = game.get_used().clone();

    let best_letter = ai.get_best_letter(prev_shown_word.to_string(), prev_used_letters);

    let correct = game.try_letter(best_letter);
    let num_right = if correct {
        let prev_count = prev_shown_word.chars().filter(|x| *x == '_').count();
        let count = game.get_shown().chars().filter(|x| *x == '_').count();
        (prev_count - count) as u8
    } else {
        0
    };

    let shown_word = game.get_shown().to_string();
    let used_letters = game.get_used();

    ai.update(
        shown_word,
        *used_letters,
        prev_shown_word.to_string(),
        prev_used_letters,
        best_letter as u8 - b'a',
        num_right,
    );
}

fn human_game(game: &mut Game) {
    loop {
        println!("{}", game.get_shown());
        println!("What is your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.len() != 2 {
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
            println!(
                "Congratz you won in {} attempts for the word {}",
                game.get_attempts(),
                game.get_word()
            );
            return;
        }
    }
}
