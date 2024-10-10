use reqwest::blocking::get;
use select::document::Document;
use select::predicate::Name;

fn main() -> Result<(), reqwest::Error> {
    // URL to scrape
    let url = "https://en.wikipedia.org/wiki/Large_language_model";

    // Make HTTP GET request
    let response = get(url)?;

    // Parse HTML content
    let body = response.text()?;

    // Parse the HTML content into a Document
    let document = Document::from(body.as_str());

    // Extract title of the page
    let title = document.find(Name("title")).next().map(|n| n.text()).unwrap_or("No title found".to_string());

    // Print the title
    println!("Title: {}", title);

    Ok(())
}
