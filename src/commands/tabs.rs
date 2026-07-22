use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tab {
    pub id: String,
    pub title: String,
    pub url: String,
    #[serde(rename = "type")]
    pub tab_type: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub websocket_url: Option<String>,
}

pub fn run() {
    // runs brave in the CDP mode
    // and just closeing the main instence is not enough
    // Every single one of the instances even in the task manager have to be closed 
    // If not it's just gonna open normal That sucks🤬🤬🤬
    // "C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe" --remote-debugging-port=9222 --remote-allow-origins=*
    let url = "http://localhost:9222/json";

    match reqwest::blocking::get(url) {
        Ok(response) => match response.text() {
            Ok(body) => {
                // Parse the raw JSON string into a vector of Tabs structs
                match serde_json::from_str::<Vec<Tab>>(&body) {
                    Ok(tabs) => {
                        println!(
                            "--- {} Tabs ---",
                            tabs.iter().filter(|tab| tab.tab_type == "page").count()
                        );

                        for (i, tab) in tabs
                            .iter()
                            .filter(|tab| tab.tab_type == "page")
                            .enumerate()
                        {
                            println!("[{}] {}", i + 1, tab.title);
                            println!("    URL: {}", tab.url);
                             println!();
                        }
                    }
                    Err(e) => eprintln!("Failed to parse JSON: {e}"),
                }
            }
            Err(e) => eprintln!("Error reading response body: {e}"),
        },
        Err(e) => {
            eprintln!("Error connecting to Brave on {url}: {e}");
        }
    }
}