pub fn get_file_contents() -> String {
    let file_contents = include_str!("../data/words.txt");
    file_contents.to_string()
}