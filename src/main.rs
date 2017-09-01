use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref OPTIONS: HashSet<i32> = [0,1,2,3,4,5,6,7].iter().cloned().collect();
}

fn main() {
    for curr_item in OPTIONS.iter() {
        println!("Hello, world! {:?}", curr_item);
    }
}
