use std::collections::HashMap;

pub struct PnCounter {
    pub increment: HashMap<String, u64>,
    pub decrement: HashMap<String, u64>,
}

impl PnCounter {
    pub fn new() -> PnCounter {
        PnCounter {
            increment: HashMap::new(),
            decrement: HashMap::new(),
        }
    }
    pub fn increment(&mut self, key: String,val:u64) {
        match self.increment.get(&key) {
            Some(value) => {
                self.increment.insert(key, value + val);
            }
            None => {
                self.increment.insert(key, val);
            }
        }
    }
    pub fn decrement(&mut self, key: String,val:u64) {
        match self.decrement.get(&key) {
            Some(value) => {
                self.decrement.insert(key, value + val);
            }
            None => {
                self.decrement.insert(key, val);
            }
        }
    }
    pub fn merge(&mut self, other: &PnCounter) {
        for (key, &other_value) in &other.increment {
            self.increment
                .entry(key.clone())
                .and_modify(|value| *value = std::cmp::max(*value, other_value))
                .or_insert(other_value);
        }
    
        for (key, &other_value) in &other.decrement {
            self.decrement
                .entry(key.clone())
                .and_modify(|value| *value = std::cmp::max(*value, other_value))
                .or_insert(other_value);
        }
    }
    pub fn total(&self) -> String {
        let  mut total_increment = 0;
        let mut total_decrement = 0;
       for (key, value) in self.increment.iter(){
        total_increment = value+total_increment;
        
        }
        for (key, value) in self.decrement.iter(){
            total_decrement = value+total_decrement;
            
        }
        (total_increment - total_decrement).to_string()
    }

    pub fn print_pn_counter(&self) {
        println!("{:#?}", &self.increment);
        println!("{:#?}", &self.decrement);
    }
}
