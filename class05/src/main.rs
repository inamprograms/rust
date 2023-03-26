// use class05::print_values;
// use class05;
fn main(){
    //creating library
    // let result:i32 = class05::integer_input_value();
    // class05::print_integer_value(result);

    //loops
    // let mut x =  1846000000;
    // loop{
    //     println!("value: {}",x);
    //     x -= 1846000000;
    // } 

    // let mut x = 0;
    // let result = loop{
    //     println!("value: {}",x);
    //     if x == 10 {
    //         break x * 2;
    //     }
    //     x += 1;
    // };
    // println!("{}",result);

    // let mut x = 0;
    // loop{
    //     println!("value: {}",x);
    //     if x == 10 {
    //         x *= 2;
    //         break; 
    //     }
    //     x += 1;
    // };
    // println!("Res: {}",x);

    // let mut x = 0;
    // loop{
    //     println!("value: {}",x);
    //     if x == 10 {
    //         x *= 2;
    //         continue; 
    //     }
    //     if x == 30 {
    //         x *= 2;
    //         break; 
    //     }
    //     x += 1;
    // };
    // println!("Res: {}",x);

    // let mut x = 0;
    // let mut y = 0;
    // 'x:loop{
    //     println!("x: {}",x);
    //     loop{
    //         println!("y: {}",y);
    //         if y == 10{
    //             y = 0;
    //             break 'x;
    //         }
    //         y += 1;
    //     }
    //     if x == 10{
    //         break;
    //     }
    //     x += 1;
    // }
    // println!("Res: {}",x);
   
    // let mut x = 0;
    // while x < 10{
    //     println!("While {}",x);
    //     x += 1;
    //     if x == 5{
    //         break;
    //     }
    // }

        let mut x = 0;
    let mut y = 0;
    'first:loop{
        println!("x: {}",x);
        'second:loop{
            println!("y: {}",y);
            y += 1;
            loop{
                println!("m: {}",m);
                z += 1;
                if z==10 {
                    continue 'second;
                }
            }
            // y += 1;
            // if z == 10{
            //     break 'first;
            // }
        }
    }
    println!("Res: {}",x);
    
}