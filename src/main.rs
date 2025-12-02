use scraper::{Selector, selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://www.rex-filmbuehne.de/inhalt/nfb/liste")?;

    // Print the HTTP response status code
    println!("Response status: {}", response.status());

    let body = response.text()?; // Fetch the response body as text
    println!("Response length: {}", body.len());

    let document = scraper::Html::parse_document(&body);
    //println!("body: {}", body);

    let selector = Selector::parse("h2 > a").unwrap();

    for element in document.select(&selector) {
        let title = element.text().collect::<String>().trim().to_string();
        println!("Film Title: {}", title.trim());
    }
    Ok(())
}
