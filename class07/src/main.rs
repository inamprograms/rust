// #[derive(Debug)]
// struct Car{ //struct Car<'a>
//     name:String,
//     color:String,
//     // color: &'a str,
//     model:u16,
//     width:u8,
//     length:u8,
//     height:u8,
// }
// impl Car {
    
//     fn dimension(&self)->u8
//     {
//         self.width*self.height*self.length
//     }
// }
// fn dimension(x:u8, y:u8,z:u8)->u8
// {
//     x*y*z
// }
// fn main() {
//     let car_01 = Car{
//         name:String::from("Mehran"),
//         color:String::from("White"),
//         // color:"White",
//         model:1992,
//         width:6,
//         length:8,
//         height:3,

//     };
//     println!("{:#?}",car_01);
//     println!("Dimension {}",dimension(car_01.width, car_01.length,car_01.height));
   
//     let car_02 = Car{
//         name:String::from("Corolla"),
//         color:String::from("Black"),
//         // color:"White",
//         model:2000,
//         width:6,
//         length:3,
//         height:4,

//     };
//     println!("{:#?}",car_02);
//     println!("Dimension {}",car_02.dimension());


// }






// struct Bus{
//     name:String,
//     no_of_seat:u8,
//     modal:u16,
//     tanksize:u16,
//     distance_cover:u16,
   
// }
// impl Bus{
//     fn millage(&self)->u16{
//         self.distance_cover / self.tanksize
//     }
// }
// fn main(){
//     let obj_bus_01 = Bus{
//     name:String::from("Mazda"),
//     no_of_seat:44,
//     modal: 1998,
//     tanksize: 23,
//     distance_cover: 500
//     };
//     println!("The millage is {}",obj_bus_01.millage());

// }




// struct Bus{
//     name:String,
//     no_of_seat:u8,
//     modal:u16,
//     tanksize:u16,
//     distance_cover:u16,
// }
// impl Bus{
   
//     fn new(
//         name:String,
//         no_of_seat:u8,
//         modal:u16,
//         tanksize:u16,
//         distance_cover:u16,)->Bus{
//         Bus{name, no_of_seat, modal,tanksize,distance_cover}
//     }
//     fn millage(&self)->u16{
//         self.distance_cover/self.tanksize
//     }
// }
// fn main(){
//     let obj_bus_02 = Bus::new(
//         String::from("Mazanda"),
//         44,
//         1998,
//         23,
//         500
//     );
//     println!("The millage is {}",obj_bus_02.millage());
// }







// struct Bus{
//     name:String,
//     // color:String,
//     no_of_seat:u8,
//     modal:u16,
//     tanksize:u16,
//     distance_cover:u16,
//     // fuel_consumption:f32,
// }
// // tanksize + distance_covered = to find milage
// impl Bus{
//     //associative -> no self
//     fn new(
       
//         name:String,
//         no_of_seat:u8,
//         modal:u16,
//         tanksize:u16,
//         distance_cover:u16,)->Bus{
//         Bus{name, no_of_seat, modal,tanksize,distance_cover}
//     }
//     // method -> pass self
//     fn millage(&self,tank_size:u16)->u16{
//         self.distance_cover/tank_size
//         // self.distance_cover/self.tanksize
//     }
// }
// fn main(){
//     // let obj_bus_01 = Bus{
//     //     name:String::from("Mazda"),
//     //     no_of_seat:44,
//     //     modal: 1998,
//     //     tanksize: 23,
//     //     distance_cover: 500
//     // };
//     // println!("The millage is {}",obj_bus_01.millage());

//     let obj_bus_02 = Bus::new(
//         // &self,
//         String::from("Mazanda"),
//         44,
//         1998,
//         23,
//         500
//     );

//     // let obj_bus = Bus{
//     //     name:String::from("Mazda"),
//     //     no_of_seat:17,
//     //     modal:1990,
//     //     tanksize:50,
//     //     distance_cover:700,
//     // };
//     // println!("The {} millage is {}",obj_bus.name,obj_bus.millage());
    
//     println!("The millage is {}",obj_bus_02.millage(100));
// }










// struct Bus{
//     name:String,
//     modal:u16,
// }

// struct  Car{
//     name:String,
//     modal:u16,
// }

// impl Car{ // for methods and associated functions
//     //method
//     fn print(&self){
//         println!("Name of Bus is {},,,{}", self.name, self.modal);
//     }
//     //associative
//     fn from(name:String, modal:u16)->Self{
//        Self{name,modal}
//     } 
// }
// fn main()
// {
//     // let obj_01 = Bus{
//     //     name:String::from("Devo"),
//     //     modal:2010,
//     // };
//     // obj_01.print();

//     // let bus_02 = Bus::init(String::from("Bvo"),2000);
// //    bus_02.print();    // let obj_02:Bus =Bus {
//     //     name:String::from("Devo"),
//     //     modal:2010,
//     // }

//     let car_02 = Car::from(String::from("Mehran"),1990);
//     car_02.print();
// }

// enum Result{
//     Ok,
//     Err,
// }
// enum  Transmition {
//     Automatic(u8,u8,),
//     Manual(u8,u8,bool),
//     Hybrid
// }

// enum Coin {
//     Head,
//     Tail
// }
// struct Car{
//     name:String,
//     modal:u16,
//     transmission:Transmition,
// }

// #[derive(Debug)]
// impl Car{
//     //method
//     fn print(&self){
//         println!("Name of Bus is {:?}", self);
//         // println!("Name of Bus is {},,,{},,,{}", self.name, self.modal, self.transmission);
//     }
//     //associative
//     fn from(name:String, modal:u16,transmission:Transmition)->Self{
//        Self{name,modal,transmission}
//     } 
// }
// fn main(){
    
    
//         let car_02 = Car::from(String::from("Mehran"),1990,Transmition::Automatic(5,100));
//         car_02.print();
// }








































































































