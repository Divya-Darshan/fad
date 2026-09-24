use crate::browser;
use console::{style, Emoji};
use crossterm::event::{self, Event, KeyCode};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tungstenite::connect;

static MUSIC_NOTE: Emoji<'_, '_> = Emoji("🎵 ", "* ");
static WAVE_ICON: Emoji<'_, '_> = Emoji("🌊 ", "~ ");

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style(" Starting Linux-Style Interactive Media Controller...").bold().cyan());

    let ws_url = browser::get_youtube_ws_url().await?;
    let (mut socket, _) = connect(&ws_url)?;

    // CDP JS script to fetch player state
    let eval_js = r#"
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
    // Linux wave-style Braille & block progress bar characters
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} {prefix} [{elapsed_precise}] ⡷⠂{bar:40.cyan/blue}⠐⢾ {pos_fmt}/{len_fmt} ({percent}%)",
        )?
        .progress_chars("⠿⠽⠾⠻⠟⠸⠼⠴⠤⠍⠅⠄"),
    );
    pb.set_prefix("Playback");

    println!("\n{}", style("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ─────────────────────────────────────────────── ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏").cyan());
    println!("{}", style(" [Space] Play/Pause  |  [N] Next Track  |  [P] Prev Track  |  [Q] Quit").bold().yellow());
    println!("{}\n", style("──────────────────────────────────────────────────────────────────").dim());

    loop {
        // 1. Listen for background keyboard events (Non-blocking)
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        println!("\n{}", style("Exiting interactive controller.").bold().red());
                        break;
                    }
                    KeyCode::Char(' ') => {
                        // Toggle play/pause via CDP WebSocket directly inside status mode!
                        let toggle_msg = json!({
                            "id": 2,
                            "method": "Runtime.evaluate",
                            "params": {
                                "expression": "const v = document.querySelector('video'); if(v) v.paused ? v.play() : v.pause();"
                            }
                        });
                        let _ = socket.send(tungstenite::Message::Text(toggle_msg.to_string().into()));
                    }
                    KeyCode::Char('n') => {
                        // Next track
                        let next_msg = json!({
                            "id": 3,
                            "method": "Runtime.evaluate",
                            "params": {
                                "expression": "const btn = document.querySelector('.ytp-next-button'); if(btn) btn.click();"
                            }
                        });
                        let _ = socket.send(tungstenite::Message::Text(next_msg.to_string().into()));
                    }
                    KeyCode::Char('p') => {
                        // Previous track
                        let prev_msg = json!({
                            "id": 4,
                            "method": "Runtime.evaluate",
                            "params": {
                                "expression": "window.history.back();"
                            }
                        });
                        let _ = socket.send(tungstenite::Message::Text(prev_msg.to_string().into()));
                    }
                    _ => {}
                }
            }
        }

        // 2. Fetch updated playback metrics from Brave CDP
        let cdp_msg = json!({
            "id": 1,
            "method": "Runtime.evaluate",
            "params": {
                "expression": eval_js,
                "returnByValue": true
            }
        });

        if socket.send(tungstenite::Message::Text(cdp_msg.to_string().into())).is_ok() {
            if let Ok(response) = socket.read() {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&response.to_string()) {
                    let result = &val["result"]["result"]["value"];
                    if !result.is_null() {
                        let title = result["title"].as_str().unwrap_or("Unknown Title");
                        let current_time = result["currentTime"].as_f64().unwrap_or(0.0) as u64;
                        let duration = result["duration"].as_f64().unwrap_or(0.0) as u64;
                        let is_paused = result["paused"].as_bool().unwrap_or(true);

                        let status_str = if is_paused {
                            style("⏸ PAUSED").yellow().bold()
                        } else {
                            style("▶ PLAYING").green().bold()
                        };

                        // Dynamic header bar refresh
                        pb.set_length(duration);
                        pb.set_position(current_time);
                        pb.set_prefix(format!("{} {}", WAVE_ICON, status_str));
                        pb.tick();
                    }
                }
            }
        }

        sleep(Duration::from_millis(250)).await;
    }

    Ok(())
}