use rand::prelude::*;

pub struct Game {
    attempts: u8,
    word: String,
    shown_word: String,
    used_letters: [bool; 26],
    finished: bool,
}

impl Game {
    pub fn new(word: String) -> Self {
        let len = word.len();
        Game {
            attempts: 0,
            word,
            shown_word: "_".repeat(len),
            used_letters: [false; 26],
            finished: false,
        }
    }

    pub fn get_shown(&self) -> &str {
        &self.shown_word
    }

    pub fn get_word(&self) -> &str {
        &self.word
    }

    pub fn get_attempts(&self) -> u8 {
        self.attempts
    }

    pub fn get_used(&self) -> &[bool; 26] {
        &self.used_letters
    }

    pub fn get_finished(&self) -> bool {
        self.finished
    }

    pub fn try_letter(&mut self, letter: char) -> bool {
        self.attempts += 1;
        let letter = letter.to_ascii_lowercase();
        let idx = (letter as u8 - b'a') as usize;
        if self.used_letters[idx] {
            return false;
        }

        self.used_letters[idx] = true;

        let mut found = false;

        for c in self.word.char_indices() {
            if c.1 == letter {
                found = true;
                self.shown_word
                    .replace_range(c.0..=c.0, &letter.to_string());
            }
        }
        
        if !self.shown_word.contains('_') {
            self.finished = true;
        }

        found
    }
}

pub fn get_random_word(file_contents: &str) -> String {
    let mut rng = rand::rng();
    let lines = file_contents.lines();
    lines.choose(&mut rng).unwrap().to_string()
}
