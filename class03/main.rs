fn main(){
    excute_me();
    get_sum(3,5);
    let (cube,number) = get_cube(5);
    println!("Cube of {} is = {}",number,cube);
    
}
fn excute_me(){
    println!("Executed");
}
fn get_sum(num1:i32,num2:i32){
    println!("Sum of {} + {} = {}",num1,num2,num1+num2);
}
fn get_cube(num:i32)->(i32,i32){
    (num*num*num,num)
}