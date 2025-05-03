
use crate::{types::*, utils};
pub struct Logger;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{BufRead, BufReader}; // <-- Fix is here
use std::fs;
use crate::utils::command::execute_command;

impl  Logger{
     pub fn log_aof(command: &Vec<&str>) {
        println!("Logging to AOF: {:?}", command); // add this
        let file_result = OpenOptions::new()
            .append(true) // Open the file in append mode
            .create(true) // Create the file if it doesn't exist
            .open("appendonly.aof");
    
        if let Ok(mut file) = file_result {
            let command_str = command.join(" ").trim().to_string(); // Join and trim
            if let Err(e) = writeln!(file, "{}", command_str) {
                eprintln!("Failed to write to AOF: {}", e);
            }
        } else {
            eprintln!("Failed to open AOF file.");
        }
    }
   pub  fn remove_aof() -> String {
        match fs::remove_file("appendonly.aof") {
            Ok(_) => "+OK File Deleted".to_string(),
            Err(e) => e.to_string(),
        }
    }

 pub fn replay_aof(db: &Db, cache: &CACHE) {
    match File::open("appendonly.aof") {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                let _ = execute_command(parts, db, cache, false);
            }
            println!("[AOF] Replay completed from appendonly.aof");
        }
        Err(_) => {
            println!("[AOF] No appendonly.aof found. Starting with empty DB.");
        }
    }
}

}