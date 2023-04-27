enum option_i32 {
    Some(i32),
    None
}
enum option_u8 {
    Some(u8),
    None
}

enum option_eneric<T> {
    Some(T),
    None
}

struct rectangle<T>{
    width:T,
    height:T
}
// [#[derive(Debug)]]
fn main() {

    // let number = option_i32::None;
    // let number = option_eneric::Some(40);
    // let number = option_eneric::Some("Inam");
    
    let rect1 =rectangle {
        width:32.4,
        height:32.6,
    };
    println!("{}", rect1.area());
    rect1.print();
    // printData(rect1);
// impl <T: rectangle<T> {
//     fn area<T:std::ops::Mul<Output = T>> (&self) -> T{
//         self.width * self.height
//     }
// }
    
    // let num = add_i32(4, 6);
    // let x:u8 =3 ;
    // let y =7 ;

    // let sum = add(x,y);
    // println!("Sum {}",sum);
}

// fn add_i32(x:i32,y:i32)->i32{
//     x+x
// }
// fn add_u8(x:u8,y:u8)->u8{
//     x+x
// }
// fn add_float32(x:f32,y:f32)->f32{
//     x+x
// }
// fn add<T: std>(x:T,y:T)->T{
//     x+x
// }


// fn printData <T>(data:val)
// {
//     println!("{}",val);
// }
fn add <T:std::ops::Add<Output = T>>(x:T,y:T)->T
{
    x+y
}

// impl <T :std::ops::Mul<Output = T>> rectangle<T>{
//     fn area(&self) -> T{
//         self.height*self.width
//     }
// }
impl rectangle<f64>{
    fn area(&self) -> f64{
        self.height*self.width
    }
    print!();
}
impl rectangle<i32>{
    fn area(&self) -> i32{
        self.height*self.width
    }

    fn print(& self){
        println!("{}",self.height);
    }
}
impl rectangle<u8>{
    fn area(&self) -> u8{
        self.height*self.width
    }
}

    fn area(&self){
        println!("{}",self.height);
    }

