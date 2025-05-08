use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};
mod types;
mod utils;
mod logging;
mod stream;
use utils::vec_utils::{join_from}; 
use utils::command::{execute_command};
use logging::logging::Logger; 
use chrono::{Duration, Utc};
use types::*;

fn handle_client(stream: TcpStream, db: Db, cache: CACHE) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;

    loop {
        let mut input = String::new();
        if reader.read_line(&mut input).is_err() {
            break;
        }
        println!("{:?}", input.trim().split_whitespace().collect::<Vec<_>>());
        println!("{:?}", db.lock().unwrap());
        println!("{:?}", cache.lock().unwrap());
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        println!("{}",parts.len());
        let response: String = execute_command(parts, &db, &cache,true);
        let _ = writer.write_all(response.as_bytes());
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6380").unwrap();
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    let cache: CACHE = Arc::new(Mutex::new(HashMap::new()));
    Logger::replay_aof(&db, &cache);
    println!("Mini Redis clone running on port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db = Arc::clone(&db);
                let cache = Arc::clone(&cache);
                thread::spawn(move || {
                    handle_client(stream, db, cache);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
