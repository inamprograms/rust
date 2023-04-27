use std::thread;
use std::time::Duration;

fn generate_workout(x: u32, y: u32) {
    let expensive_closure = |ch: char| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));

        if ch == '+'{
            x + y
        }
        else if ch == '-'{
            x - y
        }
        else if ch == '*'{
            x * y
        }
        else if ch == '/'{
            x / y
        }
        else{
            0
        }
    };
    
    println!("{} + {} = {}",x, y, expensive_closure('+'));
    println!("{} - {} = {}",x, y, expensive_closure('-'));
    println!("{} * {} = {}",x, y, expensive_closure('*'));
    println!("{} / {} = {}",x, y, expensive_closure('/'));
     
}

fn main() {
    let x = 10;
    let y = 7;

    generate_workout(x, y);
}
