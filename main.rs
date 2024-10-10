use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::error::Error;

use reqwest::blocking::get;
use select::document::Document;
use select::predicate::Name;

fn main() -> Result<(), Box<dyn Error>> {
    // URL to scrape
    let url = "https://en.wikipedia.org/wiki/Large_language_model";

    // Make HTTP GET request
    let response = get(url)?;

    // Parse HTML content
    let body = response.text()?;

    // Parse the HTML content into a Document
    let document = Document::from(body.as_str());

    // Extract main text of the page
    let mut main_text = String::new();
    for node in document.find(Name("p")) {
        main_text.push_str(&node.text());
        main_text.push_str("\n"); // Add a new line after each paragraph
    }

    // Write the extracted content to a file
    let file_path = "extracted_content.txt";
    let mut file = File::create(file_path)?;
    file.write_all(main_text.as_bytes())?;

    println!("Content extracted and saved to {}", file_path);

    Ok(())
}
