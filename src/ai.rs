use std::{collections::HashMap, hash::Hash};

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    shown_word: String,
    used_letters: [bool; 26],
}
pub struct Ai {
    q_table: HashMap<State, HashMap<u8, f64>>,
}

impl State {
    fn convert(shown_word: String, used_letters: [bool; 26]) -> Self {
        State {
            shown_word,
            used_letters,
        }
    }
}

impl Ai {
    pub fn new() -> Self {
        Ai {
            q_table: HashMap::new(),
        }
    }

    pub fn update(
        &mut self,
        shown_word: String,
        used_letters: [bool; 26],
        prev_shown_word: String,
        prev_used_letters: [bool; 26],
        letter_as_num: u8,
        num_right: u8,
    ) {
        let prev_state = State::convert(prev_shown_word, prev_used_letters);
        let state = State::convert(shown_word, used_letters);

        let points = if num_right == 0 {
            -1.0
        } else {
            num_right as f64
        };

        let prev_max = self.get_max(prev_state);
        let max = self.get_max(state.clone());

        self.q_table
            .entry(state)
            .or_insert(HashMap::new())
            .entry(letter_as_num)
            .and_modify(|val| *val = *val + 0.1 * (points + 0.9 * max.1 - prev_max.1))
            .or_insert(points);
    }

    fn get_max(&mut self, state: State) -> (u8,f64) {

        if let Some(map) = self.q_table.get_mut(&state) {
            for letter in 0..26 {
                let point = if state.used_letters[letter] {
                    f64::MIN
                } else {
                    0.0
                };
                map.entry(letter as u8).or_insert(point);
            }
            return map.iter().max_by(|a, b| a.1.total_cmp(b.1)).map(|(k,v)| (*k,*v)).unwrap();
        } else {
            self.q_table.insert(state.clone(), HashMap::new());
            let map = self.q_table.get_mut(&state).unwrap();
             for letter in 0..26 {
                let point = if state.used_letters[letter] {
                    f64::MIN
                } else {
                    0.0
                };
                map.entry(letter as u8).or_insert(point);
            }
            return map.iter().max_by(|a, b| a.1.total_cmp(b.1)).map(|(k,v)| (*k,*v)).unwrap();
        }
    }

    pub fn get_best_letter(&mut self, shown_word: String, used_letters: [bool; 26]) -> char {
        let state = State {
            shown_word,
            used_letters,
        };
        (self.get_max(state).0 + b'a') as char
    }
}
