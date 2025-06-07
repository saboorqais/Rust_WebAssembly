use crate::consumer::consumer::{Consumer, ConsumerGroup, PendingEntry};
use crate::stream::stream::{Stream, StreamFunctions};
use crate::utils::stringify::stringify_map;
use crate::utils::vec_utils::join_from;
use chrono::{DateTime, Duration, Utc};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt;
use std::fmt::Write;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
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
    Stream(Stream),
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
    fn x_read(parts: Vec<&str>, db: &Db) -> String;
    fn x_group_add(parts: Vec<&str>, db: &Db) -> String;
    fn x_group_read(parts: Vec<&str>, db: &Db) -> String;
    fn x_message_ack(parts: Vec<&str>, db: &Db) -> String;
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
    fn x_message_ack(parts: Vec<&str>, db: &Db) -> String {
        let mut db: std::sync::MutexGuard<'_, HashMap<String, RedisValue>> = db.lock().unwrap();
        let _group_name: &str = parts[1];
        let _stream_name: &str = parts[2];
        let _message_id: &str = parts[3];
        if let Some(redis_value) = db.get_mut(_stream_name) {
            let response = match &mut redis_value.value {
                ValueType::Stream(_stream) => {
                 if let Some(_consumer_group) =  _stream.consumer_groups.get_mut(_group_name){
                        _consumer_group.pending.remove(_message_id);
                       for (index,consumer)  in _consumer_group.consumers.iter_mut(){
                        if consumer.pending.contains(_message_id){
                                consumer.pending.remove(_message_id);
                        }
                       }
                 }else{

                 }
                    "+ok".to_string()
                },
                _ => "-ERR wrong type\n".to_string(),
            };

            response
        } else {
            "+Group Doesnt Exist".to_string()
        }
    }
    fn x_group_read(parts: Vec<&str>, db: &Db) -> String {
        let mut db: std::sync::MutexGuard<'_, HashMap<String, RedisValue>> = db.lock().unwrap();
        let _group_name: &str = parts[2];
        let _consumer_name: &str = parts[3];
        let mut count: Option<usize> = None;
        let _stream_map: HashMap<String, String> = HashMap::new();
        let mut stream_index = 7;
        if parts[4] == "COUNT" {
            count = parts.get(4).and_then(|s| s.parse::<usize>().ok());
        } else {
            stream_index = 4;
        }
        let stream_array = &parts[stream_index..];
        let split_index = stream_array.iter().position(|val| *val == ">");
        let index: usize = split_index.unwrap_or_else(|| panic!("Expected > in stream array"));
        let mut final_response = String::new();
        let (_stream_name_array, _stream_ids_array) = stream_array.split_at(index);
       for (index, _stream_name) in _stream_name_array.iter().enumerate() {
            if let Some(redis_value) = db.get_mut(*_stream_name) {
                let response = match &mut redis_value.value {
                    ValueType::Stream(_stream) => {
                        let response = if _stream.consumer_groups.contains_key(_group_name) {
                            let consumer_last_delivered;
                            {
                                // First mutable borrow scope
                                let _consumer_group =
                                    _stream.consumer_groups.get_mut(_group_name).unwrap();
                                let _consumer = _consumer_group
                                    .consumers
                                    .entry(_consumer_name.to_string())
                                    .or_insert_with(|| Consumer {
                                        name: _consumer_name.to_string(),
                                        last_seen: 0,
                                        pending: BTreeSet::new(),
                                    });
                                consumer_last_delivered = _consumer_group.last_delivered_id.clone();
                            } // ← _consumer_group mutable borrow ends here

                            // Now it's safe to mutably borrow _stream again
                            let response: HashMap<String, HashMap<String, String>> =
                                _stream.x_read(&consumer_last_delivered);
                            let mut latest_last_delivered_id =String::new();
                             
                                {
                            
                                    latest_last_delivered_id.push_str(_stream.entries.keys().next_back().unwrap());
                                }
                            {
                                let _consumer_group =
                                    _stream.consumer_groups.get_mut(_group_name).unwrap();
                                _consumer_group.last_delivered_id =
                                    latest_last_delivered_id.to_string();
                                for key in response.keys() {
                                    _consumer_group.pending.insert(
                                        key.to_string(),
                                        PendingEntry {
                                            entry_id: key.to_string(),
                                            delivery_count: 0,
                                            consumer_name: _consumer_name.to_string(),
                                            timestamp: Utc::now(),
                                        },
                                    );
                                }
                            }
                            {
                                let _consumer_group =
                                    _stream.consumer_groups.get_mut(_group_name).unwrap();
                                if let Some(consumer) =
                                    _consumer_group.consumers.get_mut(_consumer_name)
                                {
                                    for key in response.keys() {
                                        consumer.pending.insert(key.to_string());
                                    }
                                }
                            }

                            final_response.push_str(&stringify_map(response))
                             
                        } else {
                            final_response.push_str("Group Doesnt not Exist")
                        };
                        response
                    }

                    _ =>  final_response.push_str("Wrong Error Type"),
                };
                response
            } else {
                final_response.push_str("+OK Message")

            };
        };
      final_response
    }
    //XGROUPADD GROUP newhello mygroup 1749069781831-0
    //+New Group Added
    //XGROUPREAD GROUP mygroup alice COUNT 5 STREAMS newhello > 1749069781831-0
    fn x_group_add(parts: Vec<&str>, db: &Db) -> String {
        let mut db: std::sync::MutexGuard<'_, HashMap<String, RedisValue>> = db.lock().unwrap();
        let stream_name: &str = parts[2];
        let group_name: &str = parts[3];
        let last_delivered_id: &str = parts[4];

        if let Some(value_type) = db.get_mut(stream_name) {
            match &mut value_type.value {
                ValueType::Stream(_stream) => {
                    _stream.consumer_groups.insert(
                        group_name.to_string(),
                        ConsumerGroup {
                            name: group_name.to_string(),
                            last_delivered_id: last_delivered_id.to_string(),
                            consumers: BTreeMap::new(),
                            pending: BTreeMap::new(),
                        },
                    );
                    "+New Group Added".to_string()
                }
                _ => "-ERR wrong type\n".to_string(),
            }
        } else {
            "Error Creating Consumer Group".to_string()
        }
    }
    fn x_add(parts: Vec<&str>, db: &Db) -> String {
        let mut db: std::sync::MutexGuard<'_, HashMap<String, RedisValue>> = db.lock().unwrap();
        let key = parts[1];
        if let Some(value_type) = db.get_mut(key) {
            match &mut value_type.value {
                ValueType::Stream(_stream) => {
                    let mut hash_map: HashMap<String, String> = HashMap::new();
                    let new_chunks = parts[3..].chunks(2);
                    for chunk in new_chunks {
                        if let [key, value] = chunk {
                            hash_map.insert(key.to_string(), value.to_string());
                        }
                    }
                    _stream.add_entry(ValueType::Hash(hash_map));
                    "+New Message Added".to_string()
                }
                _ => "-ERR wrong type\n".to_string(),
            }
        } else {
            let mut new_strem = Stream::new();
            let mut hash_map: HashMap<String, String> = HashMap::new();
            let new_chunks = parts[3..].chunks(2);
            for chunk in new_chunks {
                if let [key, value] = chunk {
                    hash_map.insert(key.to_string(), value.to_string());
                }
            }
            let response = new_strem.add_entry(ValueType::Hash(hash_map));
            let redis_stream = RedisValue {
                value: ValueType::Stream(new_strem),
            };
            db.insert(key.to_string(), redis_stream);
            response
        }
    }

    fn x_read(parts: Vec<&str>, db: &Db) -> String {
        let db: std::sync::MutexGuard<'_, HashMap<String, RedisValue>> = db.lock().unwrap();
        let key = parts[2];
        if let Some(stream) = db.get(key) {
            match &stream.value {
                ValueType::Stream(_stream) => {
                    let start_id = parts[3];
                    let count: Option<usize> = parts.get(4).and_then(|s| s.parse::<usize>().ok());
                    let response = _stream.x_read(start_id);
                    let modified_response = stringify_map(response);
                    modified_response
                }
                _ => "-ERR wrong type\n".to_string(),
            }
        } else {
            "+Not Ok".to_string()
        }
    }
}

impl fmt::Display for RedisValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ValueType::String(val) => write!(f, "Result : {}", val),
            // ValueType::List(vals) => write!(f, "List({:?})", vals),
            // ValueType::Set(vals) => write!(f, "Set({:?})", vals),
            ValueType::Hash(vals) => write!(f, "Result : {:?} ", vals),
            // ValueType::SortedSet(vals) => write!(f, "SortedSet({:?})", vals),
            ValueType::LinkedList(linked_list) => write!(f, "Result : {:?} ", linked_list),
            ValueType::Stream(stream) => {
                let mut output = String::new();

                // Stream entries
                writeln!(output, "Stream Entries:").ok();
                for (id, entry) in &stream.entries {
                    if let ValueType::Hash(map) = &entry.value {
                        writeln!(output, "  ID: {}", id).ok();
                        for (k, v) in map {
                            writeln!(output, "    {} => {}", k, v).ok();
                        }
                    } else {
                        writeln!(output, "  ID: {} => {:?}", id, entry.value).ok();
                    }
                }

                // Consumer groups (only print if exists)
                if !stream.consumer_groups.is_empty() {
                    writeln!(output, "\nConsumer Groups:").ok();
                    for (group_name, group) in &stream.consumer_groups {
                        writeln!(output, "  Group: {}", group_name).ok();
                        writeln!(output, "    Last Delivered ID: {}", group.last_delivered_id).ok();

                        // Consumers
                        writeln!(output, "    Consumers:").ok();
                        for (consumer_name, consumer) in &group.consumers {
                            writeln!(output, "      Consumer: {}", consumer_name).ok();
                            writeln!(output, "        Pending IDs: {:?}", consumer.pending).ok();
                            writeln!(output, "        Last Seen: {}", consumer.last_seen).ok();
                        }

                        // Pending Entries
                        if !group.pending.is_empty() {
                            writeln!(output, "    Pending Entries:").ok();
                            for (entry_id, pending) in &group.pending {
                                writeln!(output, "      Entry ID: {}", entry_id).ok();
                                writeln!(output, "        Consumer: {}", pending.consumer_name)
                                    .ok();
                                writeln!(output, "        Timestamp: {}", pending.timestamp).ok();
                                writeln!(
                                    output,
                                    "        Delivery Count: {}",
                                    pending.delivery_count
                                )
                                .ok();
                            }
                        }
                    }
                }

                write!(f, "Stream => \n{}", output)
            }
        }
    }
}
