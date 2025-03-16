use std::collections::HashMap;

pub struct GcCounter {
    pub counter: HashMap<String, u64>,
}

impl GcCounter {
    pub fn new() -> GcCounter {
        GcCounter {
            counter: HashMap::new(),
        }
    }
    pub fn increment(&mut self, key: String) {
        match self.counter.get(&key) {
            Some(value) => {
                self.counter.insert(key, value + 1);
            }
            None => {
                self.counter.insert(key, 1);
            }
        }
    }

    pub fn value(self, key: String) -> String {
        match self.counter.get(&key) {
            Some(value) => value.to_string(),
            None => "0".to_string(),
        }
    }

    pub fn merge(&mut self, other: &GcCounter) {
        for (key, value) in self.counter.iter_mut() {
            // Mutable iteration
            if let Some(old_value) = other.counter.get(key) {
                *value = std::cmp::max(*value, *old_value); // Update value directly
            }
        }
    }
    pub fn print_gc_counter(&self) {
        println!("{:#?}", &self.counter);
    }
}
