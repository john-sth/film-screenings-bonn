pub mod fetch_data_rex_nfb;
use reqwest::Url;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let url_rex = Url::parse("https://www.rex-filmbuehne.de/inhalt/rex/liste").unwrap();
    let url_rex = "https://www.rex-filmbuehne.de/inhalt/rex/liste";
    let url_nfb = "https://www.rex-filmbuehne.de/inhalt/nfb/liste";

    let res = fetch_data_rex_nfb::fetch_data(url_nfb).expect("Error: didn't fetch data.");
    Ok(())
}
