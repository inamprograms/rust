use std::io;
pub fn print_integer_value(value:i32)
{
    println!("You entered: {}",value);
}

pub fn integer_input_value()->i32
{
    println!("Please enter the number: ");
    let mut number = String::new();
    
    io::stdin().read_line(&mut number);
    
    let number:i32 = number.trim().parse().unwrap();

    // println!("Input Number: {}",number);
    number
}