use crate::consumer::consumer::{Consumer, ConsumerGroup, PendingEntry};
use crate::types::ValueType;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
type EntryId = String;
use chrono::Utc;
#[derive(Debug)]
pub struct StreamEntry {
    pub value: ValueType,
}
pub trait StreamFunctions {
    fn new() -> Self;
    fn add_entry(&mut self, data: ValueType) -> String;
    fn x_read(&self, start_id: &str, count: Option<usize>) -> HashMap<String,HashMap<String,String>> ;
    fn x_group_add(&self, name: &str, reference_start: Option<usize>) -> String;
}
// XGROUPADD newhello worker 0
#[derive(Debug)]
pub struct Stream {
    pub entries: BTreeMap<EntryId, StreamEntry>,
    last_id: u64,
    pub consumer_groups: BTreeMap<String, ConsumerGroup>,
}

impl StreamFunctions for Stream {
    fn new() -> Self {
        Stream {
            entries: BTreeMap::new(),
            last_id: 0,
            consumer_groups: BTreeMap::new(),
        }
    }
    fn x_group_add(&self,name: &str, reference_start: Option<usize>) -> String {


        "+ok".to_string()
    }
    fn add_entry(&mut self, data: ValueType) -> String {
        let ts = Utc::now().timestamp_millis();
        let id = format!("{}-{}", ts, self.last_id);
        print!("{id}");
        self.entries.insert(id, StreamEntry { value: data });
        self.last_id = self.last_id + 1;
        "+Ok Entry Added".to_string()
    }
    fn x_read(&self, start_id: &str, count: Option<usize>) -> HashMap<String,HashMap<String,String>> {
        let mut result =HashMap::new();

        for (id, entry) in self.entries.range((
            std::ops::Bound::Excluded(start_id.to_string()),
            std::ops::Bound::Unbounded,
        )) {
            match &entry.value {
                ValueType::Hash(map) => {
                    result.insert(id.clone(), map.clone());
                }
                _ => {
                   
                }
            }
            if let Some(max) = count {
                if result.len() == max {
                    break;
                }
            }
        }
      
        result
    }
}
