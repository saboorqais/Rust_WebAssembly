 // Import the struct
 mod gc_crdt;  // Import the module
use gc_crdt::GcCounter;  //
mod or_set;
use or_set::OrSet;
use or_set::OrSetTrait;
mod pn_crdt;
use pn_crdt::PnCounter;
fn main() {
    let mut set1 = OrSet::new();
    let mut set2 = OrSet::new();

    set1.add("A".to_string(), 1);
    set1.add("B".to_string(), 2);
    set2.add("A".to_string(), 3);
    set2.add("C".to_string(), 4);
    set2.remove("A".to_string(),2);
    
    set1.merge(&set2);
    set2.merge(&set1);
    
    println!("Final OR-Set value: {:?}", set1.value());
    println!("Final OR-Set value: {:?}", set2.value());


}

