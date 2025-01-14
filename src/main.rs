#[derive(Debug)]
struct Coffe {
    id: i32,
    name: String,
    temp: TempCategory
}

#[derive(Debug)]
enum TempCategory {
    HOT(Option<f64>),
    ICED(Option<f64>)
}

fn main() {   
    println!("Start owneship_and_borrowing!");
    owneship_and_borrowing();

    println!("Start print_stack_values!");
    //print_stack_values();
}

//Second
fn owneship_and_borrowing()
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

//First
fn print_stack_values()
{
    //Integers
    let a:i32=12;
    println!("Hello, world {}!",  a);

    let parsed_num:i32 = "1234".parse().unwrap();
    println!("Parsed number: {}",  parsed_num);
    println!("Integer to string: {}",  parsed_num.to_string());

    //Floats
    let float1:f32 = 10.5;
    let float2:f32 = 10.6;
    println!("Sum of {} and {} is {}",  float1, float2, float1 + float2);
    println!("Floor: {}",  float1.floor()); //Round down
    println!("Ceiling: {}",  float1.ceil()); //Round up
    println!("Rounded: {}",  float1.round()); //Round to closest

    //Charachter
    let my_char:char = 'A';
    println!("To lowercase: {}",  my_char.to_lowercase());
    println!("To string version: {}",  my_char.to_string());

    //Booleans
    let my_bool:bool = true;
    println!("bool to string value: {}",  my_bool.to_string());
    assert_eq!(my_bool, true);

    //Tuples
    let my_tuple:(char, i32, f64) = ('A', 5, 10.5);
    println!("Tuple values {}, {}, {}.",  my_tuple.0, my_tuple.1, my_tuple.2);

    //Arrays
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    for num in my_array
    {
        println!("array value: {}",  num);
    }
    println!("First element: {}",  my_array[1]);
    println!("Items in array: {}",  my_array.len());

    //struct
    let mut my_caffe0: Coffe = Coffe {
        id: 9,
        name: String::from("Sebastjan Extra"),
        temp: TempCategory::HOT(None)
    };

    my_caffe0.id = 11;
    println!("Cafe 0 id: {}",  my_caffe0.id);

    let id:i32 = 10;
    let my_caffe: Coffe = Coffe {
            id,
            name: String::from("Sebastjan"),
            temp: TempCategory::HOT(Some(65.5))
    };

    println!("Caffe id: {}",  my_caffe.id);
    println!("Caffe name: {}",  my_caffe.name);
    println!("Caffe temp: {:?}",  my_caffe.temp);
}


