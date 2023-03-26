use std::io;

fn main() {
    // let arr = [1,2,3,3,5,6];
    // println!("arr {:?}", arr);

    let vec:Vec<i32> = Vec::new();

    for i in 0..9 {
        let mut math_number:i32;
        let mut line = String::new();
        println!("Enter your name :");
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        println!("Hello , {}", line);
        println!("no of bytes read , {}", b1);
    } 
}
