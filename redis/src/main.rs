use std::{
    collections::{HashMap,HashSet},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};
use chrono::{Utc,Duration,DateTime};
use std::fmt;


type Db = Arc<Mutex<HashMap<String, RedisValue>>>;
type CACHE = Arc<Mutex<HashMap<String, DateTime<Utc>>>>;


#[derive(Debug)]
struct LinkedList {
    value: String,
    next: Option<Box<LinkedList>>,
}

impl LinkedList {
    fn new(value: String) -> Self {
        LinkedList { value, next: None }
    }

    fn append(&mut self, value: String) {
        match &mut self.next {
            Some(next_node) => next_node.append(value),
            None => {
                self.next = Some(Box::new(LinkedList::new(value)));
            }
        }
    }
}


#[derive(Debug)]
enum ValueType {
    String(String),
    List(Vec<String>),
    Set(HashSet<String>),
    Hash(HashMap<String, String>),
    SortedSet(Vec<(f64, String)>), // (score, member)
}

#[derive(Debug)]
struct RedisValue {
    value: ValueType,
}

impl fmt::Display for RedisValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ValueType::String(val) => write!(f, "String({})", val),
            ValueType::List(vals) => write!(f, "List({:?})", vals),
            ValueType::Set(vals) => write!(f, "Set({:?})", vals),
            ValueType::Hash(vals) => write!(f, "Hash({:?})", vals),
            ValueType::SortedSet(vals) => write!(f, "SortedSet({:?})", vals),
        }
    }
}
type RedisDb = HashMap<String, RedisValue>;
fn handle_client(stream: TcpStream, db: Db,cache:CACHE) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;

    loop {
        let mut input = String::new();
        if reader.read_line(&mut input).is_err() {
            break;
        }

        println!("{:?}", input.trim().split_whitespace().collect::<Vec<_>>());
        println!("{:?}",  db.lock().unwrap());
        println!("{:?}",  cache.lock().unwrap());
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let response:String = match parts[0].to_uppercase().as_str() {
            "SET" if parts.len() == 3 => {
                db.lock()
                    .unwrap()
                    .insert(parts[1].to_string(), RedisValue { value:ValueType::String((parts[2].to_string()))  });
                cache.lock().unwrap().insert(parts[1].to_string(),Utc::now() + Duration::seconds(144000));
                "+OK\n".to_string()
            }
            "EXPIRE" if parts.len() ==3  => {
             
                let key: &str = parts[1];
                if db.lock().unwrap().get(key).is_none()  {
                    let message  =  "+Value Does Not Exist\n".to_string() ;
                    message;// Early return if key missing
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
    let cache:CACHE = Arc::new(Mutex::new(HashMap::new()));

    println!("Mini Redis clone running on port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db = Arc::clone(&db);
                let cache = Arc::clone(&cache);
                thread::spawn(move || {
                    handle_client(stream, db,cache);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
