mod basic {
    pub mod prints;
    pub mod ownership_and_borrowing;
}
use basic::ownership_and_borrowing;

use crate::basic::prints;

fn main() {   
    
    println!("Start print_stack_values!");
    prints::print_stack_values();
    //print_stack_values();

    println!("Start owneship_and_borrowing!");
    ownership_and_borrowing::owneship_and_borrowing();
}






