use crate::types::RedisValue;
use crate::types::ValueType;
use std::collections::BTreeMap;
use std::collections::HashMap;
type EntryId = String;
use chrono::Utc;
#[derive(Debug)]
pub struct StreamEntry {
   pub value: ValueType,
}
pub trait StreamFunctions {
    fn new() -> Self;
    fn add_entry(&mut self, data: ValueType) -> String;
    // fn get_entry(&self, id: &str) -> Option<&StreamEntry>;
}

#[derive(Debug)]
pub struct Stream {
   pub entries: BTreeMap<EntryId, StreamEntry>,
    last_id: u64,
}

impl StreamFunctions for Stream {
    fn new() -> Self {
        Stream {
            entries: BTreeMap::new(),
            last_id: 0,
        }
    }
    fn add_entry(&mut self, data: ValueType) -> String {
        let ts = Utc::now().timestamp_millis();
        let id = format!("{}-{}", ts, self.last_id);
        print!("{id}");
        self.entries.insert(id, StreamEntry { value: data });
        self.last_id = self.last_id +1;
        "+Ok Entry Added".to_string()
    }
}
trait ConsumerFunctions {
    fn new(name: &str) -> Self;
    fn add_pending(&mut self, id: String);
    fn ack(&mut self, id: &str) -> bool;
}
struct Consumer {
    name: String,
    pending_ids: Vec<String>, // list of unacknowledged IDs
}

struct ConsumerGroup {
    group_name: String,
    consumers: HashMap<String, Consumer>,
    last_delivered_id: u64,
    pending: HashMap<String, String>, // msg_id -> consumer_name
}
struct RedisSimulator {
    stream: Stream,
    consumer_groups: HashMap<String, ConsumerGroup>,
}
