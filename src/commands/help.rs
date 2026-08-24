pub fn run() {
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
        
        bug report: https://github.com/divya-darshan/fad
        "#
    );


}

// new set up os 