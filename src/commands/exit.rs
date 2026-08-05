use std::process::Command;


pub fn run() {
  
    let mut attempt = 0;

    println!("Stopping all Brave processes...");

    loop {
        attempt += 1;
        
        // Use Windows taskkill with /T (tree) to kill parent + all children
        let kill_output = Command::new("taskkill")
            .args(&["/F", "/T", "/IM", "brave.exe"])
            .output();

        match kill_output {
            Ok(output) => {
                if output.status.success() {
                    println!("Attempt {}: Sent kill command.", attempt);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if stderr.contains("no running instance") || stderr.contains("not found") {
                        println!("terminated");
                        break;
                    }
                }
            }
            Err(e) => eprintln!("Failed to execute taskkill: {}", e),
        }
        }
}