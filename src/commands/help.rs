pub fn run() {
    println!(
        r#"
        Usage:
        fad <command> [options]

        Common Commands:
        play        Play or resume playback
        pause       Pause playback
        next        Play the next track 400
        previous    Play the previous track 400
        start       Starts barve with CDP mode
        status      Show current playback status 400
        search      Search YouTube 400
        tabs        List browser tabs
        help        Show help information

        For more information on a command:
        fad help <command>
        
        bug report: https://github.com/divya-darshan/fad
        "#
    );


}

// new set up os 