fn main(){
    //Variable: int, float, char and constants
    // let val_in_snake_case = 10;
    // println!("Value is: {val_in_snake_case}");
    
    //Shadowing
    // let test_value = 10;
    // println!("Value : {}", test_value);
    // let test_value = test_value * 5;
    // println!("Value : {}", test_value);
    // let test_value = 2.3;
    // println!("Value : {}", test_value);
    // let test_value = '1';
    // println!("Value : {}", test_value);

    // let num1:i32 = 30;
    // let num2:i32 = 40;
    // let op:char = '+';
    // let mut result:i32 = 0;
    // if op == '+' {
    //     result = num1 + num2;
    //     println!("Addition using mut variable: {}", result);
    //     let result:i32 = num1 + num2;
    //     println!("Addition using shadowing: {}", result);
    //     // result = num1 + num2;
    //     // println!("Addition using mut variable after new shadow: {}", result);
    // }
    // else if op == '-' {
    //     let result:i32 = num1 - num2;
    //     println!("Subtraction: {}", result);
    // }
    // else if op == '*' {
    //     let result:i32 = num1 * num2;
    //     println!("Multiplication: {}", result);
    // }
    // else if op == '/' {
    //     let result:i32 = num1 / num2;
    //     println!("Division: {}", result);
    // }
    // println!("Result: {}", result);
    
    //Tuple
    // let test_tuple = ("inam",23,3.32,true);
    // // println!("Tupple value: {:?}", test_tuple);
    // println!("Tupple value: {:#?}", test_tuple);
    // let tuple = ("Ali",20,3.4);
    // let (name,age,gpa) = tuple;
    // let student_info = (name,age,gpa);
    // println!("{},{},{}", tuple.0,tuple.1,tuple.2);
    // println!("{},{},{}", name,age,gpa);
    // println!("{:?}", student_info);

    // let all_data = (test_tuple,student_info,"sufyan",23,3.7);
    // println!("{}",all_data.0.0);
    // println!("{:?}",all_data);
    // println!("{:?}",all_data.0);

    //Array
    // let roll_no = [1,2,3,4,5,6];
    // println!("{:?}",roll_no);
    // println!("{}",roll_no[0]);
    
    // let mut arr:[i8;5] = [1,0,0,0,0];
    // arr[1] = 4;
    // println!("{}",arr[1]);
    
    // let slice_arr = [1,3,5,7,9];
    // let slice = &slice_arr[0..4];
    // println!("Slice: {:?}",slice);
    
    // let multi_arr = [roll_no,[43,89,12,43,89,12]];
    // println!("Multi Array: {:?}",multi_arr);
    // println!("{}",multi_arr[1][2]);
    
    //Functions
    fn hello(){
        fn print(){
            println!("Hello World");
        }
        print(); //scope
    }
    hello();
    let mut param = 3;
    func(param);  
    param = 6;
    func(param);  
    addition(4,9);
    subtract(4,9);
    multiply(4,9);
    let first_name1:&str = "Inam ul";
    let last_name1 = "Rehman";
    let first_name = String::from("Muhammad");
    let last_name = String::from("Shafique");
    print_name_str(first_name1,last_name1);
    print_name_string(first_name,last_name);
}
fn func(value:i32){
    println!("Value: {}",value);
}
fn addition(first_num:i32, second_num:i32){
    println!(" {} + {} = {}",first_num,second_num,first_num + second_num)
}
fn subtract(first_num:i32, second_num:i32){
    println!(" {} - {} = {}",first_num,second_num,first_num - second_num)
}
fn multiply(first_num:i32, second_num:i32){
    println!(" {} * {} = {}",first_num,second_num,first_num * second_num)
}
fn print_name_str(first_name:&str,last_name:&str){ //reference type
    println!("Full Name: {} {}", first_name,last_name);
}
fn print_name_string(first_name:String,last_name:String){ //reference type
    println!("Full Name: {} {}", first_name,last_name);
}