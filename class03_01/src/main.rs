// use core::num::dec2flt::number;
// use std::num;
// use ran::*; 
// use ran::{set_seeds,RE,Rnum,Rv,Rvv};
use scanrs::scann;
 fn main() {

    // let rf = Rnum::newf64();
    // let ru = Rnum::newu64();
    // let ri = Rnum::newi64();
    // let ru16 = Rnum::newu16();
    // let ru8 = Rnum::newu8();
    // println!("Hello, world!");
    // println!("Random numbers in specified ranges: {}, {}, {}, {},{}",
    //     rf.rannum_in(0.,100.),  // wrapped f64 value 0. to 100.
    //     ru.rannum_in(1.,1000.), // wrapped u64, 1 to 1000 (inclusive)
    //     ri.rannum_in(-10.,10.), // wrapped i64, -10 to 10 (inclusive)
    //     ru16.rannum_in(60000.,65535.), // u16, 60000 to 65535
    //     ru8.rannum_in(1.,6.) // wrapped u8, 1 to 6 (inclusive)
    // );

    //CUBE
    // println!("Please input a number:");
    // let num = scann();
    // let (cub,number) = cube(num);
    // println!("cube of {} = {}",number,cub);
    
    // Control flow and scan
    println!("Enter your user name:");
    let name:String = scann();
    if name == "admin"{
        println!("Success Login");
    }
    else{
        println!("Failed to Login! Please enter your valid user name");
    }
    // get_user_name(name);

    //SUM and Control flow
    println!("Enter First number: ");
    let first_number:i32 = scann();
    println!("Enter second number: ");
    let second_number:i32 = scann();
    if first_number == second_number {
        println!("Success Login");
    }
    // println!("Sum: {} + {} = {} : ",first_number,second_number,sum(first_number,second_number));


}

// fn cube(number:i32)->(i32,i32){
//     (number*number*number,number)
// }

// fn sum(fist_num:i32,second_num:i32)->i32{
//     fist_num + second_num
// }

// fn get_user_name(user_name:&str){
//     print_user_name(user_name);
// }
// fn print_user_name(name:&str){
//     println!("Name: {}",name);
// }