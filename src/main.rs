mod basic {
    pub mod prints;
    pub mod ownership_and_borrowing;
    pub mod control_flow;
    pub mod funtions;
}
use basic::ownership_and_borrowing;
use crate::basic::prints;
use crate::basic::control_flow;
use crate::basic::funtions;

fn main() {   
    
    //println!("Start print_stack_values!");
    //prints::print_stack_values();

    //println!("Start owneship_and_borrowing!");
    //ownership_and_borrowing::owneship_and_borrowing();

    println!("Control flow!");
    let can_he_vote = control_flow::can_vote(18);
    println!("Can he vote at the age of 18? {}!", can_he_vote);

    let loop_value = control_flow::try_loop();
    println!("Loop value? {}!", loop_value);

    let while_value = control_flow::try_while();
    println!("While value: {}!", while_value);

    control_flow::try_pattern_matching(12);  

    let sum_result = funtions::anatomy(11, 2);
    println!("result: {}!", sum_result);

    let test_annonymous_result = funtions::test_annonymous(11, 2);
    println!("annonymous_result: {}!", test_annonymous_result);
}






