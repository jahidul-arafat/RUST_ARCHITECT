/*
Runtime Error
- Recoverable : file not found, use enum Result<T,E>{
    Ok(T),
    Err(E)
} // already included in the prelude
- Unrecoverable : index beyond array bounds
- RUST doesnt have exception
panic! macro
 */
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

pub fn simulation(){
    println!("Runtime Error Handling Simulation!");
    //panic!("Runtime Error Handling.. we had a problem")
    let file_to_read= "read_a_file.txt";
    let file_to_read_2= "person_names.txt";
    let numbers=[1,2,3,4,5];
    for num in numbers.iter(){
        print!("{}",num);
        //let x=1/num; // division by zero error, will panic here
        let x=divide_by_zero(1,*num);
        println!("    Div: {}",x);
    }

    // read the contents of a file named "demo_file.txt"
    // let contents = fs::read_to_string(file_to_read).unwrap(); // dont use it. Because if the file doesnt exist, unwrapping would lead to panic!. Solution is use .expect()
    //let contents = fs::read_to_string(file_to_read).expect("Something went wrong reading the"); // this would panic, if file not found and return the expect string
    // Strategy: if the file not found, then program should not panic, instead return an error message and continue
    let enum_result = fs::read_to_string(file_to_read);
    let contents = match enum_result {
        Ok(message) => message,
        //Err(err) => String::from("Error reading the file..")
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => String::from("Inside Error Kind//File not found"),
            std::io::ErrorKind::PermissionDenied => String::from("Permission denied"),
            std::io::ErrorKind::InvalidInput => String::from("Invalid input"),
            std::io::ErrorKind::AlreadyExists => String::from("File already exists"),
            _ => String::from("Error reading the file.."),
        }
    };
    /*
    Is there any better way to handle the error, other than using .expect() or unwrap() ?
    YES---> Using match expression
     */

    println!("Content is {:?}",contents); // Os { code: 2, kind: NotFound, message: "No such file or directory" }' // the file 'demo_file.txt' does not exist

    // converts the contents to lowercase
    let contents = contents.to_lowercase();
    // print the contents along with the line number
    println!("Content is {:?}",contents);
    contents
        .lines()
        .enumerate()
        .for_each(|(i,line)|{println!("Line {}: {}",i,line)});

    // check if a String 'teststring' exists in the file
    let search = input_string();
    if contents.contains(&search){
        println!("{} exists in the file", search);
    }else{
        println!(">> {} does not exist in the file",search);
    }

    // check if a file name exists else create a new file
    println!("Enter the file name to check: ");
    let file_name = input_string();
    let file_enum=check_if_file_exists(&file_name);
    match file_enum{ // this will help program to continue, without throwing any panic
        Ok(_) => println!("File {} created successfully",file_name),
        Err(err) => println!("Error: {}",err)
    }

    println!();
    println!("Merging the contents of two files");
    let result = combine_two_files(file_to_read, file_to_read_2);
    match result{
        Ok(message) => {
            // split the message into lines
            let lines: Vec<&str> = message.split('\n').collect();
            for line in lines.iter(){
                println!("{}",line);
            }

            //println!("Result: {:?}",message)
        },
        Err(err) => println!("File Merging Error: {:?}",err)
    }

}

// function to handle division by zero
pub fn divide_by_zero(x:i32,y:i32)->i32{
    if y==0{
        return 0;
    }
    x/y
}

// function to take user input as string
pub fn input_string()->String{
    println!("Please enter a string: ");
    let mut input = String::new();
    std::io::stdin()
    .read_line(&mut input)
   .expect("Failed to read line");
    input.trim().to_string().to_lowercase()
}

// function to check if a file exists or not, if not create the file
pub fn check_if_file_exists(file_name: &str) -> Result<(), std::io::Error> {
    // create file if it does not exist
    if !fs::metadata(file_name).is_ok() {
        let mut file = File::create(file_name)?;
        file.write_all(b"Flashing data into file >> program (Insider)")?;
        Ok(())
    } else {
        //println!("File already exists");
        Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists"))
    }

}

// function to read the contents of two files f1 and f2, combine them into a single output string and return Result<String, io::Error>
pub fn combine_two_files(f1: &str, f2: &str) -> Result<String, std::io::Error> {
    let mut s1 = fs::read_to_string(f1)?; // shorthand function to use // can only use this ? that returns a result enum and io error
    let mut s2 = fs::read_to_string(f2)?;
    // let mut s1 = match fs::read_to_string(f1) {
    //     Ok(s) => s,
    //     Err(e) => return Err(e), // passes the error back to the main function
    // };
    // let mut s2 = match fs::read_to_string(f2) {
    //     Ok(s) => s,
    //     Err(e) => return Err(e), // passes the error back to the main function
    // };
    s1.push_str("\n");
    s1.push_str(&s2);
    Ok(s1) // this is result Enum
}