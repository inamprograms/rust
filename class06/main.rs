struct User{
    name:String,
    email:String,
    user_id:i32,
    active: bool,
    sign_in_count:u32,

}
fn main()
{
    // let mut user1 = User{
    //     email:String::from("inam@gmail.com"),
    //     name:String::from("Inam"),
    //     user_id:34,
    // };
    // user1.name = String::from("Inam ul rehamn");
    // println!("{}",user1.name);

    let name = String::from("Inam");
    let email = String::from("Inam@gmail.com");
    fn create_user(name:String, email:String)->User
    {
        User{
            name:name,
            email,
            // email:email,
            active:true,
            user_id:234,
            sign_in_count: 1,
        }
    }
    let user2 = create_user(name,email);
    println!("{}",user2.name);
    println!("{}",user2.email);
    println!("{}",user2.active);
    println!("{}",user2.sign_in_count);
    println!("{}",user2.user_id);

}