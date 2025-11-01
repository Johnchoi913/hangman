use crate::clean::clean_file;

mod clean;

fn main() {
    match clean_file("data/words.txt") {
        Ok(_) => println!("File cleaned"),
        Err(e) => eprintln!("Error cleaning {}", e),
    }
}

