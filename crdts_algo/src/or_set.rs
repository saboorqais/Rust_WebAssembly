use std::collections::HashMap;
use std::collections::HashSet;
pub trait OrSetTrait {
    fn new() -> OrSet;
    fn add(&mut self, key: String, val: u64);
    fn remove(&mut self, key: String, val: u64);
    fn merge(&mut self, other: &OrSet);
 fn value(&self)->HashSet<String>;
    // fn print_or_set(&self);
}

pub struct OrSet {
    pub additions: HashMap<String, HashSet<u64>>,
    pub removals: HashMap<String, HashSet<u64>>,
}


impl OrSetTrait for OrSet {
    fn new() -> OrSet {
        OrSet {
            additions: HashMap::new(),
            removals: HashMap::new(),
        }
    }
    fn add(&mut self, key: String, val: u64) {
        match self.additions.get_mut(&key) {
            Some(set) => {
                set.insert(val);
            }
            None => {
                let mut new_set = HashSet::new();
                new_set.insert(val);
                self.additions.insert(key, new_set);
            }
        }
    }
    fn remove(&mut self, key: String, val: u64) {
        match self.removals.get_mut(&key) {
            Some(set) => {
                set.insert(val);
            }
            None => {
                let mut new_set = HashSet::new();
                new_set.insert(val);
                self.additions.insert(key, new_set);
            }
        }
    }
    fn merge(&mut self, other: &OrSet) {
        for (element, timestamps) in &other.additions {
            self.additions
                .entry(element.clone())
                .or_default()
                .extend(timestamps);
        }
        for (element, timestamps) in &other.removals {
            self.removals
                .entry(element.clone())
                .or_default()
                .extend(timestamps);
        }
    }
    fn value(&self) ->HashSet<String>{
        self.additions.iter().filter_map(|(element, timestamps)| {
            if let Some(removed_timestamps) = self.removals.get(element) {
                let active_timestamps: HashSet<_> = timestamps.difference(removed_timestamps).collect();
                if !active_timestamps.is_empty() {
                    return Some(element.clone());
                }
            } else {
                return Some(element.clone());
            }
            None
        }).collect()
    }
}
