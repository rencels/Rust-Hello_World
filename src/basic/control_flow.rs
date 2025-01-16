pub fn can_vote(age: i8) -> String
{
    if age > 17 {
        String::from("Yes! He can!")
    }
    else {
        String::from("No he cannot")
    }    
}

pub fn try_loop() -> i32
{
    let mut counter:i32 = 0;

    loop {
        if counter==3 { break;};  //break exists the loop
        counter += 1;
    }

    counter
}

pub fn try_while() -> i32
{
    let mut counter:i32 = 0;

    while counter < 20 {        
        counter += 1;
    }

    counter
}

enum MessageStatus {
    Pending(i32),
    Sending(i32),
    Received(i32),
}

//USE match 
pub fn try_pattern_matching(status_code: i32) {
    let msg_state = MessageStatus::Pending(status_code);

    match msg_state {
        MessageStatus::Pending(code) => print!("Message Pending with code: {}", code),
        MessageStatus::Sending(code) => print!("Message Sending with code: {}", code),
        MessageStatus::Received(code) => print!("Message Received with code: {}", code),
        _ => print!("Message is no longer pending!"),
    }

    
}


