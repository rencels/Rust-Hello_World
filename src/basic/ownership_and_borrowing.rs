//Second
pub fn owneship_and_borrowing()
{
    let num1:i16 = 2;
    let num2:i16 = 4;
    println!("The first number where I am owner {}!",  num1);
    println!("The second number where I am owner {}!",  num2);

    let result:i32 = summarize(num1, num2);
    println!("Sum:{}!",  result);

    //string
    let name: String = String::from("Sebastjan");
    let _name2 = take_string_ownership(name);
    //will not work, because the ownership or name is tranfered to _name2   println!("Original string {}!",  name);
    println!("The new owned string {}!",  _name2);

    let string_to_borrow: String = String::from("Sebastjan borrow");
    println!("The borrow string value {}!",  string_to_borrow);
    let returned_value = borrow_string( &string_to_borrow);
    println!("Returned value {}!",  returned_value);

    let string_to_decorate: String = String::from("Sebastjan to decorate");
    println!("The value to decorate: {}!",  string_to_decorate);
    let decorated_string = decorate_string(string_to_decorate);
    println!("The decorated value: {}!",  decorated_string);
}

fn summarize(num1:i16, num2:i16) -> i32 {
    i32::from(num1) + i32::from(num2)
}

fn take_string_ownership(my_string:String) -> String {
    my_string
}

fn borrow_string(my_string:&String) -> &String {
    println!("The value of the borrowed value {}!",  my_string);
    my_string
}

fn decorate_string(my_string:String) -> String {
    let result = my_string + "END";
    return result;
}