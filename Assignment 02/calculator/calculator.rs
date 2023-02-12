use std::io;
fn main(){
    let mut number1;
    let mut number2;
    let mut ch;
    println!("Enter the first number: ");
    number1 = String::new();
    io::stdin().read_line(&mut number1);
    let number1:f64 = match number1.trim().parse()
    {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),  
    };
  

    println!("Enter the second number: ");
    number2 = String::new();
    io::stdin().read_line(&mut number2);
    let number2:f64 = match number2.trim().parse()
    {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),  
    };
    
    
    println!("Op: ");
    ch = String::new();
    io::stdin().read_line(&mut ch);
    let ch:char = match ch.trim().parse()
    {
        Ok(num) => num,
        Err(_) => panic!("Not a character!"),  
    };
    if ch == '+'{
        println!("Result: {} + {} = {}", number1, number2, number1 + number2);
    }
    else if ch == '-' {
        println!("Result: {} - {} = {}", number1, number2, number1 - number2);   
    }
    else if ch == '*' {
        println!("Result: {} * {} = {}", number1, number2, number1 * number2);   
        
    }
    else if ch == '/' {
        println!("Result: {} / {} = {}", number1, number2, number1 / number2);   
    }
    else if ch == '%' {
        println!("Result: {} % {} = {}", number1, number2, number1 % number2);     
    }
    else{
        println!("Invalid operator! Enter the following valid operators +, -, *, /, % ");     
    }
}