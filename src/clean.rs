use std::fs::{File, remove_file, rename};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

pub fn clean_file(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let new_file = File::create(format!("{}_new", file_path))?;
    let reader = BufReader::new(file);
    let mut writer = BufWriter::new(new_file);

    for line in reader.lines() {
        if let Ok(line) = line
            && !line.contains(|a: char| !a.is_ascii_alphabetic())
        {
            writeln!(writer, "{}", line.to_ascii_lowercase())?;
        }
    }

    remove_file(file_path)?;

    rename(format!("{}_new", file_path), file_path)?;

    Ok(())
}
