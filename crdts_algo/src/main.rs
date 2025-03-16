 // Import the struct
 mod gc_crdt;  // Import the module
use gc_crdt::GcCounter;  //

mod pn_crdt;
use pn_crdt::PnCounter;
fn main() {
    let mut gc_counter = PnCounter::new();
    let mut gc_counter_2 = PnCounter::new();

    gc_counter.increment("a".to_string(),5);
    gc_counter_2.increment("b".to_string(),3);
    gc_counter_2.decrement("b".to_string(),2);


    

     gc_counter.merge(&gc_counter_2);
     gc_counter_2.merge(&gc_counter);
     gc_counter.print_pn_counter();
     gc_counter_2.print_pn_counter();
     println!("Total: {}", gc_counter.total());
    println!("Total: {}", gc_counter_2.total());


}

