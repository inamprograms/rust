pub trait Specification{
    // fn mobile(&self,brand:String) -> String;
    // fn laptop(&self,brand:String) -> String;
    // fn desktop(&self,brand:String) -> String;
    fn device(&self) -> String; // if this is individual then it will be function
    fn screen_size(&self) -> i32; 
}//Above whole mechanism depend on object/ is associated with object 

pub struct Laptop{
    pub processor: String,
    pub ram: String,
    pub storage: String,
    pub length: i32,
    pub width: i32,

}

impl Specification for Laptop{
    fn device(&self) -> String {
        format!("The processor: {}\n The RAM: {}\n The Storage: {}",
        self.processor,self.ram,self.storage) 
    }
    fn screen_size(&self) -> i32 {
        self.length * self.width
    }
    
}
pub struct desktop{
    pub processor: String,
    pub ram: String,
    pub storage: String,
    pub lcd_size: String,
    pub mouse: bool,
    pub keyboard: bool,
    pub webcam: bool,
}

fn main() {
    let hp440g7 = Laptop{
        processor: String::from("i7-10500u"),
        ram: String::from("8 GB"),
        storage: String::from("512 GB"),
        length: 10,
        width: 12,
    };

    println!("HP 844-G7:\n {} \n With dimension: {}",
    hp440g7.device(), hp440g7.screen_size());
}
