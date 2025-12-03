use reqwest::Url;
use scraper::{Selector, selector};

pub fn fetch_data(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;

    // Print the HTTP response status code
    println!("Response status: {}", response.status());

    let body = response.text()?; // Fetch the response body as text
    println!("Response length: {}", body.len());

    let document = scraper::Html::parse_document(&body);
    //println!("body: {}", body);

    //===========================================================
    // filter after the css selectors
    // currently title is inside an a tag, which itself
    // is in a h2 tag
    //===========================================================
    let block_selector = Selector::parse(".nfb_liste").unwrap();
    let selector = Selector::parse("h2 > a").unwrap();
    let date_selector = Selector::parse(".vorstellung_text").unwrap();
    let dir_selector = Selector::parse("h4").unwrap();
    let film_descr = Selector::parse(".filmbox span").unwrap();

    //===========================================================
    // iterate through the whole document and print the movie titles
    //===========================================================
    // get the text nodes of the element, join them to a String
    // Object and trim for whitespaces
    // trim returns a &str slice, which borrows the original String
    // trim doesnt own the data
    // to_string allocates a new String and copies the slice into it
    //===========================================================

    let movie_data = document.select(&block_selector);

    for movie in movie_data {
        let title = movie
            .select(&selector)
            .next()
            .map(|d| d.text().collect::<String>().trim().to_string())
            .unwrap();
        println!("Film Title: {}", title);
        let screenings = movie.select(&date_selector);
        for (index, screening) in screenings.enumerate() {
            let text = screening.text().collect::<String>().trim().to_string();
            println!("screening: {}. {}", index + 1, text);
        }
        let director = movie
            .select(&dir_selector)
            .next()
            .map(|d| d.text().collect::<String>().trim().to_string())
            .unwrap();
        println!("Director: {}", director);
        let description = movie
            .select(&film_descr)
            .next()
            .map(|d| d.text().collect::<String>().trim().to_string())
            .unwrap();
        println!("Description: {}", description);
    }
    Ok(())
}
