use core::panic;
use std::{env, error::Error, fs};

pub fn run(pdf_path: String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(pdf_path)?;

    // Parse the PDf file
    let parsed = parser::parse(&contents);

    // Create the Anki deck
    let deck = ankify::create_deck(parsed);

    // Save the deck
    let path = env::current_dir()?;
    let path = path.join("output.apkg");
    let path = path.to_str().unwrap();
    let result = deck.save(path);

    Ok(result)
}

mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
