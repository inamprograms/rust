fn main(){
    // Variables
    let mut x = 5;
    println!("The value of x is:{}",x);
    x = 6;
    println!("The value of x is:{}",x);
    //Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is:{}",MAX_POINTS);
    //Shadowing
    let mut y = 5;
    println!("Value of y: {}", y);
    y = 2;
    println!("Value of y after mut: {}", y);
    let y = "inam";
    println!("Value of y: {}", y);
    //Integer data type
    let d:u8 = 25;
    println!("Value of d: {}", d);
    let s:i8 = -128;
    println!("Value of s: {}", s);


}


