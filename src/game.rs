pub struct Game {
    attempts: u8,
    word: String,
    shown_word: String,
    used_letters: [bool; 26],
}

