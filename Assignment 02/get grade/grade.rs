// break , return of num: how these works ?
// handle bounds like corss of 100percent + handle total numbers 
// and more..
//Test/evaluate
use std::io;
fn main()
{
    loop
    {
        println!("Enter your no of subjects: ");
        let mut no_of_subjects = String::new();
        io::stdin().read_line(&mut no_of_subjects);
        let no_of_subjects:u32 = match no_of_subjects.trim().parse(){
            Ok(num) => {
                println!("{}", no_of_subjects);
                let obtained_marks:f32 = enter_marks_of_subjects(num);
                let percentage = print_result(obtained_marks);
                get_grade(percentage);
                num

            },
            Err(_) =>{ 
                println!("Expected like 10, 2, 3");
                0
            },
        };
       
    } 
}

fn enter_marks_of_subjects(no_of_subjects:u32)->f32
{
    let mut obtained_marks:f32 = 0.0;
    for subject in 0..no_of_subjects {
            println!("Enter marks of subject {}:",subject+1);
            let mut subject_marks = String::new();
            io::stdin().read_line(&mut subject_marks);
            let subject_marks:f32 = match subject_marks.trim().parse(){
                Ok(num) => num,
                Err(_) => {
                    println!("Not a number");
                    0.0
                },
            };
            obtained_marks += subject_marks;
    }
    obtained_marks
}

fn print_result(obtained_marks:f32)->i32{
    let total_marks:f32 = 400.0;
    let result = ( obtained_marks /total_marks)*100.0;
    println!("Your result: {}%", result);
    result as i32
}

fn get_grade(percentage:i32){

    if(percentage >= 80 && percentage <= 100){
        println!("Your grade is A with {}% marks.",percentage);
    }
    else if(percentage >= 60 && percentage < 80){
        println!("Your grade is B with {}% marks.",percentage);
    }
    else if(percentage >= 40 && percentage < 60){
        println!("Your grade is C with {}% marks.",percentage);   
    }
    else if(percentage >= 25 && percentage < 40){
        println!("Your grade is D with {}% marks.",percentage);   
    }
    else if(percentage >= 25 && percentage < 40){
        println!("Your grade is D with {}% marks.",percentage);   
    }
    else{
        println!("Your grade is F with {}% marks.",percentage);   
    }
}