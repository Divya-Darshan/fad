mod commands;

use std::env;

fn main() {
    // Collect all command line arguments
    let args: Vec<String> = env::args().collect();

    // If no command is given, print help
    if args.len() < 2 {
        println!(
r#"
Usage:
    fad <command> [options]

Common Commands:
    play        Play or resume playback
    pause       Pause playback
    next        Play the next track
    previous    Play the previous track
    status      Show current playback status
    search      Search YouTube
    tabs        List browser tabs
    help        Show help information

For more information on a command:
    fad help <command>
"#
        );

        // bug report: https://github.com/divya-darshan/fad
        return;
    }

    // The command is the second argument
    let command = &args[1];

    // Execute the appropriate command
    match command.as_str() {
        "play" => commands::play::run(),
        "pause" => commands::pause::run(),
        "next" => commands::next::run(),
        "previous" => commands::pre::run(),
        "status" => commands::status::run(),
        "search" => commands::search::run(),
        "help" => commands::help::run(),
        "tabs" => commands::tabs::run(),

        _ => {
            println!("Unknown command: {}", command);
            println!("Type 'fad help' to see available commands.");
        }
    }
}