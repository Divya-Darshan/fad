use crate::browser;
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tungstenite::connect;

static PLAY_ICON: Emoji<'_, '_> = Emoji("▶ ", "> ");
static PAUSE_ICON: Emoji<'_, '_> = Emoji("⏸ ", "|| ");
static MUSIC_NOTE: Emoji<'_, '_> = Emoji("🎵 ", "* ");

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style(" Fetching live media status from Brave...").bold().cyan());

    let ws_url = browser::get_youtube_ws_url().await?;
    let (mut socket, _) = connect(&ws_url)?;

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

    let pb = ProgressBar::new(100);
    let mut initialized = false;

    // Live update loop
    loop {
        let cdp_msg = json!({
            "id": 1,
            "method": "Runtime.evaluate",
            "params": {
                "expression": js_code,
                "returnByValue": true
            }
        });

        socket.send(tungstenite::Message::Text(cdp_msg.to_string().into()))?;

        let response = socket.read()?;
        let val: serde_json::Value = serde_json::from_str(&response.to_string())?;
        let result = &val["result"]["result"]["value"];

        if !result.is_null() {
            let title = result["title"].as_str().unwrap_or("Unknown Title");
            let current_time = result["currentTime"].as_f64().unwrap_or(0.0) as u64;
            let duration = result["duration"].as_f64().unwrap_or(0.0) as u64;
            let is_paused = result["paused"].as_bool().unwrap_or(true);

            if !initialized {
                let status_icon = if is_paused { PAUSE_ICON } else { PLAY_ICON };
                let status_str = if is_paused {
                    style("PAUSED").yellow().bold()
                } else {
                    style("PLAYING").green().bold()
                };

                println!("\n{} {} {}", status_icon, MUSIC_NOTE, style(title).bold().magenta());
                println!("Status: {}\n", status_str);

                pb.set_length(duration);
                pb.set_style(
                    ProgressStyle::with_template(
                        "{prefix} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)",
                    )?
                    .progress_chars("█▉▊▋▌▍▎▏  "),
                );
                pb.set_prefix("Playback");
                initialized = true;
            }

            pb.set_position(current_time);
        }

        // Poll every 500ms
        sleep(Duration::from_millis(500)).await;
    }
}