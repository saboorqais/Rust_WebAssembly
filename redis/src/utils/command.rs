use std::fs::{File, OpenOptions};
use std::io::{ Write};
use crate::types::*;

use std::io::{BufReader, BufRead}; // <-- Fix is here


pub fn log_aof(command: &str) {
    println!("Logging to AOF: {}", command); // add this
    let file_result = OpenOptions::new()
        .append(true)   // Open the file in append mode
        .create(true)   // Create the file if it doesn't exist
        .open("appendonly.aof");

    if let Ok(mut file) = file_result {
        if let Err(e) = writeln!(file, "{}", command.trim()) {
            eprintln!("Failed to write to AOF: {}", e);
        }
    } else {
        eprintln!("Failed to open AOF file.");
    }
}


pub fn execute_command(parts: Vec<&str>, db: &Db, cache: &CACHE) -> String {
    if parts.is_empty() {
        return "-ERR empty command\n".to_string();
    }

    match parts[0].to_uppercase().as_str() {
        "SET" if parts.len() >= 3 => RedisValue::set(parts, db, cache),
        "LPUSH" if parts.len() == 3 => RedisValue::lpush(parts, db, cache),
        "LPOP" if parts.len() == 2 => RedisValue::lpop(parts, db, cache),
        "EXPIRE" if parts.len() == 3 => RedisValue::expire(parts, db, cache),
        "GET" if parts.len() == 2 && parts[1] == "*" => RedisValue::get_all(db),
        "GET" if parts.len() == 2 => RedisValue::get_key(parts, db),
        "DEL" if parts.len() == 2 => RedisValue::remove(parts, db),
        "EXIT" => "EXIT".to_string(), // for handling shutdown in server
        _ => "-ERR unknown or unsupported command\n".to_string(),
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
                let _ = execute_command(parts, db, cache);
            }
            println!("[AOF] Replay completed from appendonly.aof");
        }
        Err(_) => {
            println!("[AOF] No appendonly.aof found. Starting with empty DB.");
        }
    }
}