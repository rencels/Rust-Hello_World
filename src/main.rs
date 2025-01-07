fn main() {        
    print_stack_values();
}

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


