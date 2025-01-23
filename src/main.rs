mod basic {
    pub mod prints;
    pub mod ownership_and_borrowing;
    pub mod control_flow;
    pub mod funtions;
    pub mod error_handling;
}

use basic::ownership_and_borrowing;
use crate::basic::prints;
use crate::basic::control_flow;
use crate::basic::funtions;
use basic::funtions::Point;
use basic::error_handling;

fn main() {   
    
    //1
    //println!("Start print_stack_values!");
    //prints::print_stack_values();

    //2 Owners and borrowers
    //println!("Start owneship_and_borrowing!");
    //ownership_and_borrowing::owneship_and_borrowing();

    //3 Control flow
    //println!("Control flow!");
    //let can_he_vote = control_flow::can_vote(18);
    //println!("Can he vote at the age of 18? {}!", can_he_vote);

    //let loop_value = control_flow::try_loop();
    //println!("Loop value? {}!", loop_value);

    //let while_value = control_flow::try_while();
    //println!("While value: {}!", while_value);

    //control_flow::try_pattern_matching(12);  

    //4 Functions
    //let sum_result = funtions::anatomy(11, 2);
    //println!("result: {}!", sum_result);

    //let test_annonymous_result = funtions::test_annonymous(43, 2);
    //println!("annonymous_result: {}!", test_annonymous_result);

    //let mut new_point = Point::new(10, 5);
     
    //new_point.print();
    //new_point.add(7);
    //new_point.print();

    //5 error handling
    error_handling::Open_File();

    let valid_result = error_handling::multiply_str_by_ten("100");
    let abs_result = error_handling::multiply_str_by_ten("abs");

}






