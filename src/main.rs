use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

const SIZE: i32 = 8;
const QUEEN: &'static str = "\u{265B}";

lazy_static! {
    static ref OPTIONS: HashSet<i32> = [0,1,2,3,4,5,6,7].iter().cloned().collect();
}

fn print_board() {
    let line = "+---+---+---+---+---+---+---+---+";
  
    for row_position in OPTIONS.iter() {
        println!("{}", line);;
        for i in 0..SIZE {
            print!("| ");

            if *row_position == i {
                print!("{} ", QUEEN);
            }
            else {
                print!("  ");
            }
        }
        print!("|\n");
    }
    println!("{}", line);;
}

fn main() {
    print_board();
}
