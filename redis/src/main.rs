use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};
mod types;
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

        let response: String = match parts[0].to_uppercase().as_str() {
            "SET" if parts.len() == 3 => {
                db.lock().unwrap().insert(
                    parts[1].to_string(),
                    RedisValue {
                        value: ValueType::String((parts[2].to_string())),
                    },
                );
                cache
                    .lock()
                    .unwrap()
                    .insert(parts[1].to_string(), Utc::now() + Duration::seconds(144000));
                "+OK\n".to_string()
            }
            "LPUSH" if parts.len() == 3 => {
                let key = parts[1];
                let value = parts[2].to_string();
                let mut db = db.lock().unwrap();

                if let Some(redis_value) = db.get_mut(key) {
                    match &mut redis_value.value {
                        ValueType::LinkedList(list) => {
                            list.append(value);
                            "+OK\n".to_string()
                        }
                        _ => "-ERR wrong type\n".to_string(),
                    }
                } else {
                    let mut list = LinkedList::new(value);
                    db.insert(
                        key.to_string(),
                        RedisValue {
                            value: ValueType::LinkedList(list),
                        },
                    );
                    cache
                        .lock()
                        .unwrap()
                        .insert(key.to_string(), Utc::now() + Duration::seconds(144000));
                    "+Key Created\n".to_string()
                }
            }
            "LPOP" if parts.len() == 3 => {
                let key = parts[1];
                let value = parts[2].to_string();
                let mut db = db.lock().unwrap();

                if let Some(redis_value) = db.get_mut(key) {
                    match &mut redis_value.value {
                        ValueType::LinkedList(list) => {
                            list.append(value);
                            "+OK\n".to_string()
                        }
                        _ => "-ERR wrong type\n".to_string(),
                    }
                } else {
                    let mut list = LinkedList::new(value);
                    db.insert(
                        key.to_string(),
                        RedisValue {
                            value: ValueType::LinkedList(list),
                        },
                    );
                    cache
                        .lock()
                        .unwrap()
                        .insert(key.to_string(), Utc::now() + Duration::seconds(144000));
                    "+Key Created\n".to_string()
                }
            }
            "EXPIRE" if parts.len() == 3 => {
                let key: &str = parts[1];
                if db.lock().unwrap().get(key).is_none() {
                    let message = "+Value Does Not Exist\n".to_string();
                    message; // Early return if key missing
                }

                let mut cache = cache.lock().unwrap();
                let duration = Duration::seconds(parts[2].parse::<i64>().unwrap());

                let later = match cache.get(key) {
                    Some(existing) => *existing + duration,
                    None => Utc::now() + duration,
                };

                cache.insert(key.to_string(), later);

                "+OK\n".to_string()
            }
            "GET" if parts.len() == 2 && parts[1] == "*" => {
                let db = db.lock().unwrap();
                if db.is_empty() {
                    "$-1\n".to_string()
                } else {
                    let mut response = String::new();
                    for (k, v) in db.iter() {
                        response.push_str(&format!("{}: {}\n", k, v));
                    }
                    response
                }
            }
            "GET" if parts.len() == 2 => match db.lock().unwrap().get(parts[1]) {
                Some(val) => format!("+{}\n", val),
                None => "$-1\n".to_string(),
            },
            "DEL" if parts.len() == 2 => {
                let removed = db.lock().unwrap().remove(parts[1]);
                format!(":{}\n", if removed.is_some() { 1 } else { 0 })
            }
            "EXIT" => break,
            _ => "-ERR unknown command\n".to_string(),
        };

        let _ = writer.write_all(response.as_bytes());
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6380").unwrap();
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    let cache: CACHE = Arc::new(Mutex::new(HashMap::new()));

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
