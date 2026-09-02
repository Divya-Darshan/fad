//src/browser.rs
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use tungstenite::connect;

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

/// Opens a new browser tab with the specified target URL
pub fn open_new_tab(target_url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get("http://localhost:9222/json")?;
    let tabs: Vec<Tab> = response.json()?;

    // Find any open page tab with an active WebSocket URL
    let ws_url = tabs
        .into_iter()
        .find(|t| t.tab_type == "page" && t.websocket_url.is_some())
        .and_then(|t| t.websocket_url)
        .ok_or("No active browser tab found. Make sure Brave is running with 'fad start'")?;

    // Connect via WebSocket and send the CDP Target.createTarget command
    let (mut socket, _) = connect(ws_url)?;

    let msg = json!({
        "id": 1,
        "method": "Target.createTarget",
        "params": {
            "url": target_url
        }
    });

    socket.send(tungstenite::Message::Text(msg.to_string().into()))?;
    Ok(())
}