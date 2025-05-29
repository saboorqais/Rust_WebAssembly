use std::collections::{BTreeMap,BTreeSet};
#[derive(Debug)]
pub struct ConsumerGroup {
    pub name: String,
    pub last_delivered_id: String, // last entry ID delivered to this group
    pub consumers: BTreeMap<String, Consumer>, // consumer name -> Consumer struct
    pub pending: BTreeMap<String, PendingEntry>, // entry ID -> PendingEntry (unacknowledged)
}
#[derive(Debug)]
pub struct Consumer {
    pub name: String,
    pub pending: BTreeSet<String>, // list of entry IDs assigned to this consumer and not yet acked
    pub last_seen: u64, // optional timestamp to expire idle consumers
}
#[derive(Debug)]
pub struct PendingEntry {
    pub entry_id: String,
    pub consumer_name: String,
    pub timestamp: u64, // delivery timestamp
    pub delivery_count: u32, // number of times this entry was delivered
}
