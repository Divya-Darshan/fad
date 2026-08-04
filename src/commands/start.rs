use std::process::{Command};


pub fn run() { 
    // define the path to Brave this is mine 🤓☝️
       let brave_path = r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe";
   
       // launch Brave with remote debugging enabled
       let mut child = Command::new(brave_path)
           .arg("--remote-debugging-port=9222")
           .arg("--remote-allow-origins=*")
           .spawn()
           .expect("Failed to start Brave Browser");
   
       println!("Brave started with PID: {}", child.id());

}