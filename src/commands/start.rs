use std::process::Command;
use std::{thread, time::Duration};
use sysinfo::System;
use std::ffi::OsStr;

pub fn run() {
    if let Err(e) = restart_brave_cdp() {
        eprintln!("Error: {}", e);
    }
}

fn restart_brave_cdp() -> Result<(), Box<dyn std::error::Error>> {
    let brave_name = OsStr::new("brave.exe");
    let max_attempts = 5;
    let mut attempt = 0;

    println!("Stopping all Brave processes...");

    // Aggressively kill all Brave processes and in some causes it loops never stops
    // no idea why?
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
                        println!("All Brave processes already closed.");
                        break;
                    }
                }
            }
            Err(e) => eprintln!("Failed to execute taskkill: {}", e),
        }

        // Wait for OS to clean up handles
        thread::sleep(Duration::from_secs(2));

        // Verify if any processes are still alive
        let mut system = System::new_all();
        system.refresh_all();
        let remaining: Vec<_> = system
            .processes_by_name(brave_name)
            .collect();

        if remaining.is_empty() {
            println!("terminated");
            break;
        }

        println!(" {} processes still alive, retrying...", remaining.len());

        if attempt >= max_attempts {
            eprintln!("ERROR: Could not kill all Brave processes after {} attempts.", max_attempts);
            return Err("Failed to terminate Brave processes".into());
        }
    }

    println!("Starting Brave in CDP mode...");
    thread::sleep(Duration::from_secs(2));

   // define the path to Brave this is mine 🤓☝️
    let brave_path = r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe";
    
           // launch Brave with remote debugging enabled
           let mut child = Command::new(brave_path)
               .arg("--remote-debugging-port=9222")
               .arg("--remote-allow-origins=*")
               .spawn()
               .expect("Failed to start Brave Browser");

    println!("Brave started in CDP mode with PID: {}", child.id());
    
    Ok(())
}   

//use std::process::{self, Command};


// pub fn run() { 
//     // define the path to Brave this is mine 🤓☝️
//        let brave_path = r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe";
   
//        // launch Brave with remote debugging enabled
//        let mut child = Command::new(brave_path)
//            .arg("--remote-debugging-port=9222")
//            .arg("--remote-allow-origins=*")
//            .spawn()
//            .expect("Failed to start Brave Browser");
   
//        println!("Brave started with PID: {}", child.id());

// }