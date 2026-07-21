// src/commands/tabs.rs

pub fn run() {
    let url = "http://localhost:9222/json";

    match reqwest::blocking::get(url) {
        Ok(response) => match response.text() {
            Ok(body) => println!("{body}"),
            Err(e) => eprintln!("Error reading response body: {e}"),
        },
        Err(e) => {
            eprintln!("Error connecting to Brave on {url}: {e}");
        }
    }
}