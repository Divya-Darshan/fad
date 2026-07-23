use serde::Deserialize;
use std::error::Error;

#[allow(dead_code)]
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

/// Fetches open tabs and returns the YouTube tab's WebSocket URL
pub fn get_youtube_ws_url() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("http://localhost:9222/json")?;
    let tabs: Vec<Tab> = response.json()?;

    for tab in tabs {
        if tab.tab_type == "page" && tab.url.contains("youtube.com") {
            if let Some(ws_url) = tab.websocket_url {
                return Ok(ws_url);
            }
        }
    }

    Err("No open YouTube tab found with an active WebSocket debugger URL.".into())
}