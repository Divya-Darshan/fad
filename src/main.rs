mod browser;
mod commands;

use std::env;

// runs brave in the CDP mode
// and just closeing the main instence is not enough
// Every single one of the instances even in the task manager have to be closed
// If not it's just gonna open normal That sucks🤬🤬🤬 I have no idea Y
// "C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe" --remote-debugging-port=9222 --remote-allow-origins=*
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
            start       Starts barve with CDP mode
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
        "start" => commands::start::run(),
        "status" => commands::status::run(),
        "search" => commands::search::run(),
        "exit" => commands::exit::run(),
        "help" => commands::help::run(),
        "tabs" => commands::tabs::run(),

        //install beave
        "winbrave" => commands::winbrave::run(),
        _ => {
            println!("Unknown command: {} what you yapping bout bruh!", command);
            println!("Type 'fad help' to see available commands.");
        }
    }
}
