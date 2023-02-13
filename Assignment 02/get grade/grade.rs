use std::io;
fn main(){
    let mut obtained_marks:f32 = 0.0;
    println!("Enter your no of subjects: ");
    let mut no_of_subjects = String::new();
    io::stdin().read_line(&mut no_of_subjects);
    let no_of_subjects:i32 = match no_of_subjects.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("Not a number"),
    };
    
    for subject in 0..no_of_subjects {
        println!("Enter marks of subject {}:",subject);
        let mut subject_marks = String::new();
        io::stdin().read_line(&mut subject_marks);
        let subject_marks:f32 = match subject_marks.trim().parse(){
            Ok(num) => num,
            Err(_) => panic!("Not a number 1"),
        };
        obtained_marks += subject_marks;
    }

    let total:f32 = 400.0;
    let result = ( obtained_marks /total)*100.0;
    println!("Your result: {}%", result);

    
}