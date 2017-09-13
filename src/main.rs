use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

const SIZE: usize = 8;
const QUEEN: &'static str = "\u{265B}";

type BoardType = Vec<i32>;

lazy_static! {
    static ref OPTIONS: HashSet<i32> = [0,1,2,3,4,5,6,7].iter().cloned().collect();
}

fn print_board(board: &BoardType) {
    let line = "+---+---+---+---+---+---+---+---+";
  
    for row_position in board {
        println!("{}", line);
        for i in 0..SIZE {
            print!("| ");

            if *row_position == i as i32 {
                print!("{} ", QUEEN);
            }
            else {
                print!("  ");
            }
        }
        print!("|\n");
    }
    println!("{}", line);
}

fn new_board() -> BoardType {
    vec![0; SIZE as usize]
}

fn do_recurse(board: &mut BoardType, index: usize) -> bool {
    let mut taken_values : HashSet<i32> = HashSet::new();

    for i in 0..index {
        let diff = (index - i) as i32;
        let value = board[i];
        taken_values.insert(value);
        taken_values.insert(value - diff);
        taken_values.insert(value + diff);
    }

    let remaining = OPTIONS.difference(&taken_values);

    for option in remaining {
        board[index] = *option;

        if index == SIZE - 1 || do_recurse(board, index + 1) {
            return false;
        }
    }

    return false;
}

fn recusive_solve(board: &mut BoardType) {
    do_recurse(board, 0);
}

fn main() {
    let mut board = new_board();
    recusive_solve(&mut board);
    print_board(&board);
}
