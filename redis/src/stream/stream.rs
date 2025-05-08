use crate::types::RedisValue;
use std::collections::HashMap;
use std::collections::BTreeMap;
type EntryId = String;
#[derive(Debug)]
pub struct StreamEntry {
    value: RedisValue,
}
#[derive(Debug)]
pub struct Stream {
    entries: BTreeMap<EntryId, StreamEntry>,
    last_id:u64,
}
pub trait StreamFunctions {
    fn new() -> Self;
    // fn add_entry(&mut self, data: String) -> String;
    // fn get_entry(&self, id: &str) -> Option<&StreamEntry>;
}

impl StreamFunctions for Stream {
    fn new()->Self {
        Stream {
            entries: BTreeMap::new(),
            last_id:0

        }
    }
    // fn add_entry(&mut self, data: RedisValue) -> Stream{
        
    // }

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