use scanrs::scann;
fn main() {
    loop {
        
        println!("Enter first number: ");
        let first_number:i32 = scann();
        println!("Enter second number: ");
        let second_number:i32 = scann();
        println!("Operator: ");
        let op:char = scann();
    
        if op == '+'{
            println!("Result: {} + {} = {}", first_number, second_number, first_number + second_number);
        }
        else if op == '-' {
            println!("Result: {} - {} = {}", first_number, second_number, first_number - second_number);   
        }
        else if op == '*' {
            println!("Result: {} * {} = {}", first_number, second_number, first_number * second_number);   
            
        }
        else if op == '/' {
            println!("Result: {} / {} = {}", first_number, second_number, first_number / second_number);   
        }
        else if op == '%' {
            println!("Result: {} % {} = {}", first_number, second_number, first_number % second_number);     
        }
        else{
            println!("Invalid operator! Enter the following valid operators +, -, *, /, % ");     
        }
    }
}
