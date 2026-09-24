use crate::browser;
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::json;
use std::thread;
use std::time::Duration;
use tungstenite::connect;

static PLAY_ICON: Emoji<'_, '_> = Emoji("▶ ", "> ");
static PAUSE_ICON: Emoji<'_, '_> = Emoji("⏸ ", "|| ");
static MUSIC_NOTE: Emoji<'_, '_> = Emoji("🎵 ", "* ");

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style(" Fetching media status from Brave...").bold().cyan());

    // 1. Get YouTube WebSocket Debugger URL from browser module
    let ws_url = browser::get_youtube_ws_url()?;
    let (mut socket, _) = connect(&ws_url)?;

    // 2. JS Snippet to retrieve title, currentTime, duration, and paused state from YouTube player
    let js_code = r#"
        (() => {
            const video = document.querySelector('video');
            const titleElem = document.querySelector('h1.ytd-watch-metadata yt-formatted-string');
            if (!video) return null;
            return {
                title: titleElem ? titleElem.innerText : document.title,
                currentTime: video.currentTime || 0,
                duration: video.duration || 0,
                paused: video.paused
            };
        })()
    "#;

    let cdp_msg = json!({
        "id": 1,
        "method": "Runtime.evaluate",
        "params": {
            "expression": js_code,
            "returnByValue": true
        }
    });

    socket.send(tungstenite::Message::Text(cdp_msg.to_string().into()))?;

    // 3. Receive WebSocket evaluation response
    let response = socket.read_message()?;
    let val: serde_json::Value = serde_json::from_str(&response.to_string())?;

    let result = &val["result"]["result"]["value"];
    if result.is_null() {
        println!("{}", style("No active video found on YouTube.").red());
        return Ok(());
    }

    let title = result["title"].as_str().unwrap_or("Unknown Title");
    let current_time = result["currentTime"].as_f64().unwrap_or(0.0) as u64;
    let duration = result["duration"].as_f64().unwrap_or(0.0) as u64;
    let is_paused = result["paused"].as_bool().unwrap_or(true);

    // Format header with console styles
    let status_icon = if is_paused { PAUSE_ICON } else { PLAY_ICON };
    let status_str = if is_paused {
        style("PAUSED").yellow().bold()
    } else {
        style("PLAYING").green().bold()
    };

    println!("\n{} {} {}", status_icon, MUSIC_NOTE, style(title).bold().magenta());
    println!("Status: {}", status_str);

    if duration == 0 {
        println!("{}", style("Live stream or duration unavailable.").dim());
        return Ok(());
    }

    // 4. Create customized ASCII progress bar using Indicatif
    let pb = ProgressBar::new(duration);
    pb.set_position(current_time);

    // ASCII block style layout
    pb.set_style(
        ProgressStyle::with_template(
            "{prefix} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos_fmt}/{len_fmt} ({percent}%)",
        )?
        .progress_chars("██-"),
    );

    pb.set_prefix("Playback");

    // Display progress
    pb.finish_with_message("Status updated");

    println!("\n{}", style("──────────────────────────────────────────────────").dim());

    Ok(())
}