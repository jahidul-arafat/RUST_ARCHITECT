use std::fs;
use std::io::prelude::*;

pub fn simulation(){
    //read_from_file();
    //write_to_file();
    append_to_file();

}

fn read_from_file() {
    println!("Read Data from file");
    // read the contents of the file named "read_a_file.txt"
    let contents = fs::read_to_string("read_a_file.txt").expect("Something went wrong reading the file");
    println!("Contents of the file:\n{}", contents);
    contents
        .lines()
        .enumerate()
        .for_each(|(index, line)| println!("Line {}, {}", index, line));


    // // define an array to 10 integers
    // let mut array: [i32; 10] = [0; 10];
    // // fill the array with 10 integers
    // for i in 0..10 {
    //     array[i] = i as i32;
    // }
    // // print the array
    // for i in &array {
    //     println!("{}", i);
    // }

    //[1,2,3].iter().for_each(|x| println!("Value: {}", x));

    // yield the lines of the string one by one
    // for line in contents.lines() {
    //     println!("{}", line);
    // }

    // read the contents of the file named "read_a_file.txt" as bytes, not as string
    let contents = fs::read("read_a_file.txt").expect("Something went wrong reading the file"); // a vector of bytes in the file
    println!("Contents of the file:\n{:?}", contents)
}

// overwrite the existing contents of a file
fn write_to_file() {
    let mut speech = String::new();
    speech.push_str("Hello, ");
    speech.push_str("world!");
    speech.push_str("\n");
    speech.push_str("Jahidul Arafat");
    fs::write("write_to_file.txt", speech).expect("Something went wrong writing the file");
    println!("Wrote to file");
}

// method to append to a file named "read_a_file.txt"
fn append_to_file() {
    // open the file named "read_a_file.txt" in append mode
    let mut file = fs::OpenOptions::new().append(true).open("read_a_file.txt").expect("Something went wrong opening the file");
    file.write(b"\nAppended Line 01").expect("TODO: panic message");
    println!("File Appended");
}