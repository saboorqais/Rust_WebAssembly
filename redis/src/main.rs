use std::{
    collections::{HashMap},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};
#[macro_use]
mod macros;
mod types;
mod utils;
mod logging;
mod stream;
mod consumer;
use utils::command::execute_command;
use logging::logging::Logger; 
use types::*;
use std::env;
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
    let port = env::var("PORT").unwrap_or_else(|_| "127.0.0.1:6381".to_string());
    println!("Mini Redis clone running on port {}",&port);
    let listener = TcpListener::bind(port).unwrap();
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    let cache: CACHE = Arc::new(Mutex::new(HashMap::new()));
    Logger::replay_aof(&db, &cache);
    for stream in listener.incoming() {
        println!("Client Connected");
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
