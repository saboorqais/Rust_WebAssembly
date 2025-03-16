use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut gc_counter = GcCounter::new();
    let mut gc_counter_2 = GcCounter::new();

    gc_counter.increment("a".to_string());
    gc_counter.increment("b".to_string());
    gc_counter.increment("c".to_string());
    gc_counter_2.increment("c".to_string());
    gc_counter_2.increment("c".to_string());

    gc_counter.merge(&gc_counter_2);
    gc_counter.print_gc_counter();


}

struct GcCounter {
    counter: HashMap<String, u64>,
}

impl GcCounter {
    fn new() -> GcCounter {
        GcCounter {
            counter: HashMap::new(),
        }
    }
    fn increment(&mut self,key:String){
        match self.counter.get(&key) {
            Some(value)=>{
                self.counter.insert(key,value+1) ;
         
            }
            None => {
                self.counter.insert(key,1) ;
            }
        }

        
            }

    fn value(self, key: String) -> String {
        match self.counter.get(&key) {
            Some(value) => value.to_string(),
            None => "0".to_string(),
        }
    }

    fn merge(&mut self,  other: &GcCounter) {
        for (key, value) in self.counter.iter_mut() { // Mutable iteration
            if let Some(old_value) = other.counter.get(key) {
                *value = std::cmp::max(*value, *old_value); // Update value directly
            }
        }
    }
    fn print_gc_counter(&self){
        println!("{:#?}", &self.counter);
    }
}
