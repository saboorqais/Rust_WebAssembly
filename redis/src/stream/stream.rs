use crate::vec_utils::RedisValue;
use std::collections::HashMap;
struct StreamEntry {
    id: String,         // e.g., "1-0", "2-0"
    data: RedisValue,       // or use a custom key-value struct if supporting fields
}
struct Stream {
    entries: Vec<StreamEntry>,
    last_id: u64,
}
trait StreamFucntions {
    fn new() -> Self;
    fn add_entry(&mut self, data: String) -> String;
    // fn get_entry(&self, id: &str) -> Option<&StreamEntry>;
}
impl StreamFucntions for Stream {
    fn new() {
        Stream {
        }
    }
    fn add_entry(&mut self, data: RedisValue) -> Stream{

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