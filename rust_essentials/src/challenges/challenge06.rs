use std::{fs, io};
use std::io::prelude::*;

pub fn challenge() {
    let file_name = "person_names.txt";
    let user_input = take_user_input();
    let mut found_status: bool=false;

    // read the contents of the file
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    // contents
    //     .lines()
    //     .for_each(|line| println!("{}", line));

    // contents
    //     .lines()
    //     .for_each(|line|
    //         if line.contains(&user_input) {println!("Found it!"); found_status=true;});

    // iterates over the lines of the file and check if the line contains the string "John"
    for line in contents.lines() {
        //println!("Matching {}::{}", line,user_input);
        if line.contains(&user_input) {
            println!("{} is in the file",user_input);
            found_status = true;
            return
        }
    }

    // if the search string is not in the file, then append it to the end of the file
    println!("{} is not in the file",user_input);
    // append this to the end of the file
    let mut file = fs::OpenOptions::new().append(true).open(file_name).expect("Something went wrong opening the file");
    file.write(format!("\n{}",user_input).as_bytes()).expect("Failed to write to the file");

}


// method to take user input
pub fn take_user_input()-> String {
    let mut input = String::new();
    println!("Please enter your name: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered: {}", input);
    input.trim().parse().unwrap()
}
