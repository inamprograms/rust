fn main() {
    // Option enum[Some, None] and Result enum[Ok, Err] are premitive types not compound types like string objects(String::from()) will not work as like string literal(&str)
//     let int = Some(5);
//     let flt = Some(5.0);
//     let bool = Some(true);
//     let ch = Some('a');

// // Generic Types In Function Definitions
//     fn largest<T:std::cmp::PartialOrd>(list: &[T]) -> &T {
//         let mut largest = &list[0];
    
//         for item in list {
//             if item > largest {
//                 largest = item;
//             }
//         }
    
//         largest
//     }
    
    
//     let number_list = vec![34, 50, 25, 100, 65];
    
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
    
//     let char_list = vec!['y', 'm', 'a', 'q'];
    
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
    
//     // Generic Type In Struct Definitions
//     #[derive(Debug)]
//     struct Point<T, U> {
//         x :T,
//         y: U,
//     }
    
//     let integer_val = Point{x: 5, y: 10};
//     println!("{:#?}", integer_val);
//     let float_val = Point{x: 5.0, y:10.0};
//     println!("{:#?}", float_val);
//     let int_float_val = Point{x: 5, y:10.0};
//     println!("{:#?}", int_float_val);

//     // Generic Type In Enum Definitions

//     enum Option <T>{
//         Some(T),
//         None,
//     }
//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
    
//     // Generic Type In Method Definitions
//     impl <T, U> Point<T, U> {
//         fn get_x(&self) -> &T {
//             &self.x 
//         }
//         fn get_y(&self) -> &U {
//             &self.y
//         }
//     }
//     let res = Point{x:10, y:20.1};
//     println!("Result: {}", res.get_x());
//     println!("Result: {}", res.get_y());
    
//     impl Point<i32, f32>{
//         fn print_values(&self){
//             println!("X: {}, Y: {}", &self.x, &self.y);
//         }
//     }
//     let val = Point{x:88, y:99.9};
//     val.print_values();
    

    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }
    
    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
                                    //x1    y1
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);    
}
