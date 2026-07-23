use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use tungstenite::connect;
use tungstenite::Utf8Bytes;
use tungstenite::Message;

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

// ctrl c and v in the paly from pause 🤓☝️
pub fn run() { 
    // Find the YouTube tab WebSocket URL
    let ws_url = match get_youtube_ws_url() {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Error finding YouTube tab: {e}");
            return;
        }
    };

    // Connect to the WebSocket
    let (mut socket, _response) = match connect(&ws_url) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to connect to WebSocket: {e}");
            return;
        }
    };

    // Construct the CDP Runtime.evaluate command payload
    let cdp_command = json!({
        "id": 1,
        "method": "Runtime.evaluate",
        "params": {
            "expression": "document.querySelector('video').play()"
        }
    });

    // 4. Send the payload over the WebSocket (using `send` instead of `write_message`)
    let msg = Message::Text(Utf8Bytes::from(cdp_command.to_string()));
    if let Err(e) = socket.send(msg) {
        eprintln!("Failed to send pause command: {e}");
        return;
    }

    println!("playing");
}

/// Helper function to fetch open tabs and return the YouTube WebSocket URL
fn get_youtube_ws_url() -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::get("http://localhost:9222/json")?;
    
    // Requires reqwest feature "json"
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