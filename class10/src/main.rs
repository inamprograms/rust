// use std::collections::HashMap;
use std::io;

// fn main() {
//     let name = String::from("name");
//     let email = String::from("email");
//     let mut user_map:HashMap<String,String>= HashMap::new();
//     user_map.insert(name.clone(), String::from("inam"));
//     user_map.insert(email.clone(), String::from("inam@gmail.com"));
//     // user_map.insert(String::from("email"), String::from("inamul@gmail.com"));
//     user_map.insert(String::from("password"), String::from("12345"));

//     println!("User email {:?}", user_map.get(& email));
//     user_map.remove(& email);
//     user_map.entry(email.clone()).or_insert(String::from("test@mail"));  
//     println!("User email {:?}", user_map.get(& email));
    
//     // println!("Map {:#?}", user_map);
//     // println!("User Name {:?}", user_map.get(& name));
//     //class task to get with out some
//     // match user_map.get(& email) {
//     //     Some(string)=> println!("email {}", string),
//     //     None => panic!("Enamil not fond") // for unrecoverabale error , program will not work, if user enered wrong input  we want  to stope our program there we do not want to go ahead all next to panic will not be executed, panic done at runtime,
//     // }
//     // user_map.remove(& name);
    
//     // user_map.remove(& email);
//     // println!("User email {:?}", user_map.get(& email));
//     println!("Map {:#?}", user_map);

// }



// fn main()
// {
//     let email = String::from("email");
//     let mut user_map:HashMap<String,String>= HashMap::new();
//     user_map.insert(email.clone(), String::from("inam@gmail.com"));
//     println!("Map {:#?}", user_map);
//     println!("Map email value{:?}", & user_map["email"]);

// }



// use std::{fs::File, io::{ErrorKind, Write}};

fn main() {
    // result enum
    // option enum
    file_check_create();
    match read_file{
        Ok(string)=> println!("String {}",string),
        Err(e)=> panic!("Error in file, {:?}",e),
    }
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     //result enum 1. ok 2. err
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // match greeting_file_result {
    //     Ok(file)=> println!("File found {:?}",file),
    //     // Err(err)=> 
    //     Err(err)=>match err.kind() {
    //         ErrorKind::NotFound=> match File::create("hello.txt"){
    //         Ok(_)=> println!("File created"),
    //         Err(e)=>println!("Error in file creating {:?}",e)
    //     }
    //     _=> panic!("File create err")
    //     }

    // }

//     // let mut data = find_or_create();
//     // println!("{:?}",data);
//     // match find_or_create(){
//     //     Ok(string)=> println!("{:?}",string)
//     //     Err(err)
//     // }

   

}

fn read_file()-> Result<String,io::Error>
{
    greeting_file_result
}

// // fn find_or_create()-> Result<String,io::Error>
// // {
// //     let greeting_file_result = File::open("hello.txt");
// //     let mut file = match  greeting_file_result{
// //         Ok(file)=> file,
        
// //     };
// // }

fn file_check_create(){
    let greeting_file_result = File::open("hello.txt");
    match greeting_file_result {
        Ok(file)=> println!("File found {:?}",file),
        // Err(err)=> 
        Err(err)=>match err.kind() {
            ErrorKind::NotFound=> match File::create("hello.txt"){
                Ok(mut f)=> {
                    f.write(b"hell inam fiale data");
                    println!("File created")
                }
                Err(e)=>panic!("Error in file creating {:?}",e),
            },
            _=> panic!("File create err"),
        },
    }  
}


// fn main()
// {
//     let mut vector = vec![2,3,4,5,6,7,8,9,0];
//     vector[8] = 1000;

//     println!("Vector {:?}",vector);
//     println!("Vector index {:?}",vector.get(20)); //get is: )option enum(: 1.sum 2.none

//         // Result enum{
//         //     Ok(task),
//         //     Err(error_kind),
//         // }program will panic if result enum is not used, at runtime

// }
