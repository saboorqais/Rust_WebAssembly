use crate::utils::vec_utils::join_from;
use crate::stream::stream::{Stream,StreamFunctions};
use chrono::{DateTime, Duration, Utc};
use std::fmt;
use std::{
    collections::{HashMap},
    sync::{Arc, Mutex}
};
use std::fmt::Write;
// Declare the `utils` module
// Access `vec_utils` from the `utils` module

pub type Db = Arc<Mutex<HashMap<String, RedisValue>>>;
pub type CACHE = Arc<Mutex<HashMap<String, DateTime<Utc>>>>;

#[derive(Debug)]
pub struct LinkedList {
    pub value: String,
    pub next: Option<Box<LinkedList>>,
}

impl LinkedList {
    pub fn new(value: String) -> Self {
        LinkedList { value, next: None }
    }
    pub fn pop(&mut self) -> Option<String> {
        match self.next.as_mut() {
            Some(next_node) if next_node.next.is_none() => {
                // The next node is the last node
                let last_node = self.next.take().unwrap();
                Some(last_node.value)
            }
            Some(_) => {
                // Recurse until we reach second last node
                self.next.as_mut().unwrap().pop()
            }
            None => {
                // This is a single-element list; cannot pop itself
                // Optional: return Some(self.value.clone()) and mark this node as empty (?)
                if self.value.is_empty() {
                    None
                } else {
                    let old_value = self.value.clone();
                    self.value = String::new(); // Clear the current value
                    self.next = None; // Ensure it's "empty"
                    Some(old_value)
                }
            }
        }
    }
    pub fn append(&mut self, value: String) {
        match &mut self.next {
            Some(next_node) => next_node.append(value),
            None => {
                self.next = Some(Box::new(LinkedList::new(value)));
            }
        }
    }
}

#[derive(Debug)]
pub enum ValueType {
    String(String),
    // List(Vec<String>),
    // Set(HashSet<String>),
    Hash(HashMap<String, String>),
    // SortedSet(Vec<(f64, String)>),
    LinkedList(LinkedList),
    Stream(Stream)
}

#[derive(Debug)]
pub struct RedisValue {
    pub value: ValueType,
}
pub trait RedisFunctions {
    fn set(parts: Vec<&str>, db: &Db, cache: &CACHE) -> String;
    fn remove(parts: Vec<&str>, db: &Db) -> String;
    fn lpush(parts: Vec<&str>, db: &Db, cache: &CACHE) -> String;
    fn lpop(parts: Vec<&str>, db: &Db, cache: &CACHE) -> String;
    fn expire(parts: Vec<&str>, db: &Db, cache: &CACHE) -> String;
    fn get_all(db: &Db) -> String;
    fn get_key(parts: Vec<&str>, db: &Db) -> String;
    fn x_add(parts: Vec<&str>, db: &Db) -> String;
}
impl RedisFunctions for RedisValue {
    fn remove(parts: Vec<&str>, db: &Db) -> String {
        let removed = db.lock().unwrap().remove(parts[1]);
        let response = format!(":{}\n", if removed.is_some() { 1 } else { 0 });
        response
    }
    fn get_key(parts: Vec<&str>, db: &Db) -> String {
        match db.lock().unwrap().get(parts[1]) {
            Some(val) => format!("+{}\n", val),
            None => "$-1\n".to_string(),
        }
    }
    fn get_all(db: &Db) -> String {
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

    fn expire(parts: Vec<&str>, db: &Db, cache: &CACHE) -> String {
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
    fn set(parts: Vec<&str>, db: &Db, cache: &CACHE) -> String {
        db.lock().unwrap().insert(
            parts[1].to_string(),
            RedisValue {
                value: ValueType::String(join_from(&parts, 2)),
            },
        );
        cache
            .lock()
            .unwrap()
            .insert(parts[1].to_string(), Utc::now() + Duration::seconds(144000));
        "+OK\n".to_string()
    }
    fn lpush(parts: Vec<&str>, db: &Db, cache: &CACHE) -> String {
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
            let list = LinkedList::new(value);
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
    fn lpop(parts: Vec<&str>, db: &Db, cache: &CACHE) -> String {
        let key = parts[1];
        let mut db: std::sync::MutexGuard<'_, HashMap<String, RedisValue>> = db.lock().unwrap();
        if let Some(redis_value) = db.get_mut(key) {
            match &mut redis_value.value {
                ValueType::LinkedList(list) => {
                    match list.pop() {
                        Some(response) => {
                            if list.value.is_empty() {
                                db.remove_entry(key);
                            }
                            response
                        }
                        None => {
                            db.remove_entry(key);
                            "-ERR empty list\n".to_string()
                        } // <- Handle empty list here
                    }
                }
                _ => "-ERR wrong type\n".to_string(),
            }
        } else {
            "-Key Does not Exist\n".to_string()
        }
    }
    fn x_add(parts: Vec<&str>, db: &Db) -> String {
        let mut db: std::sync::MutexGuard<'_, HashMap<String, RedisValue>> = db.lock().unwrap();
        let key = parts[1];
        if let Some(value_type ) = db.get_mut(key){
            match &mut value_type.value {
                ValueType::Stream(_stream) => {
                    let mut  hash_map:HashMap<String,String> =  HashMap::new();
                    let new_chunks = parts[3..].chunks(2);
                    for chunk in new_chunks{
                        if let [key, value] = chunk {
                            hash_map.insert(key.to_string(), value.to_string());
                        }
                    }
                    _stream.add_entry(ValueType::Hash(hash_map));
                    "+New Message Added".to_string()
                },
                _ =>  "-ERR wrong type\n".to_string(),
            
            }
        }else{
        let mut new_strem = Stream::new();
        let mut  hash_map:HashMap<String,String> =  HashMap::new();
        let new_chunks = parts[3..].chunks(2);
        for chunk in new_chunks{
            if let [key, value] = chunk {
                hash_map.insert(key.to_string(), value.to_string());
            }
        }
        let response =new_strem.add_entry(ValueType::Hash(hash_map));
        let redis_stream  = RedisValue { 
            value:ValueType::Stream(new_strem)
        };
        db.insert(key.to_string(),redis_stream );
        response
        }

    }
}

impl fmt::Display for RedisValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ValueType::String(val) => write!(f, "String({})", val),
            // ValueType::List(vals) => write!(f, "List({:?})", vals),
            // ValueType::Set(vals) => write!(f, "Set({:?})", vals),
            ValueType::Hash(vals) => write!(f, "Hash({:?})", vals),
            // ValueType::SortedSet(vals) => write!(f, "SortedSet({:?})", vals),
            ValueType::LinkedList(linked_list) => write!(f, "LinkedList({:?})", linked_list),
            ValueType::Stream(stream) => {
                let mut output = String::new();
                for (id, entry) in &stream.entries {
                    if let ValueType::Hash(map) = &entry.value {
                        let _ = writeln!(output, "{}:", id);
                        for (k, v) in map {
                            let _ = writeln!(output, "  {} => {}", k, v);
                        }
                    }
                }
                write!(f, "Stream => {}", output) // This returns the formatted stream
            }
        
        }
    }
}

