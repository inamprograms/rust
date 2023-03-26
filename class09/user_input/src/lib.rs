pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub mod output{
    pub fn print_hello(x:i32){
        // println!("heooll inam");
        println!("heooll inam {}",square(x));
        // println!("heooll inam {}",crate::output02::square(x));
    }

    fn square(x:i32)->i32 //private
    {
        x * x
    }

    
}
// mod output02{
//     pub fn square(x:i32)->i32 //private
//     {
//         x * x
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
