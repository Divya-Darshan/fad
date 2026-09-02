use websearch::{web_search, providers::DuckDuckGoProvider, SearchOptions};

pub async fn run(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for: {}", query);

    let duckduckgo = DuckDuckGoProvider::new();

    let results = web_search(SearchOptions {
        query: query.to_string(),
        max_results: Some(5),
        provider: Box::new(duckduckgo),
        ..Default::default()
    }).await?;

    for r in &results {
        println!("{}: {}", r.title, r.url);
    }

    Ok(())
}