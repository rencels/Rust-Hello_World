use std::num::ParseIntError;
use std::fs;

pub fn Open_File()
{
    let open_file_result = fs::read_to_string("C:\\Users\\SH_ADMIN_DEV38\\Desktop\\TRAMS\\TestDoc.txt");

    match open_file_result {
        Ok(testfile) => println!("We successfully opened the file"),
        Err(err) => panic!("Can't open file")
    }
}

pub fn multiply_str_by_ten(a: &str) -> Result<i32, ParseIntError> {
    let parsed_a:i32 = a.parse()?;
    Ok(parsed_a * 10)

}