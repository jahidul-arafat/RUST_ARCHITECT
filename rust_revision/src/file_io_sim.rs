use std::{fs, io};
use std::io::Write; // to read and write to file

pub(crate) fn simulation(){
    println!("File IO Simulation");
    let file_to_read = String::from("read_a_file.txt");
    let file_to_write = "write_a_file.txt";
    let file_to_append = "append_a_file.txt";
    read_from_file_as_string(&file_to_read);
    println!("File Name: {}", file_to_read); // to check if borrowed or ownership transferred; ownership transfer must be avoided


    universal_read_as_bytes_using_vector(&file_to_read);
    println!();
    write_to_a_file(file_to_write);

    println!();
    append_to_a_file(file_to_append);
}

fn read_from_file_as_string(file_to_read: &str){
    println!("File IO Read");
    let contents = fs::read_to_string(file_to_read)
        .expect("Something went wrong reading the file");
    // println!("{}", contents);
    //
    // for line in contents.lines() {
    //     println!("{}", line);
    // }
    // println!("--------------------------------");

    contents
        .lines()
        .enumerate()
        .for_each(|(index, line)| println!("{}: {}", index, line));
}

fn universal_read_as_bytes_using_vector(file_to_read: &str) {
    let contents = fs::read(file_to_read).expect("Something went wrong reading the file");
    println!("Contents: {:?}", contents);
}

fn write_to_a_file(file_to_write: &str){
    // let mut contents_buffer = String::new();
    // contents_buffer.push_str("This is the first line\n");
    // contents_buffer.push_str("This is the second line\n");
    // contents_buffer.push_str("This is the third line\n");
    // contents_buffer.push_str("This is the fourth line\n");

    let contents = get_user_input_as_string();
    println!("Contents: {}", contents);

    fs::write(file_to_write, contents)
        .expect("Something went wrong writing to the file");

}

// method to append to a file
fn append_to_a_file(file_to_append: &str){
    let contents = get_user_input_as_string();
    //println!("Contents: {}", contents);

    // open the file in append mode
    let mut opened_file_to_append = fs::OpenOptions::new()
        .append(true)
        .open(file_to_append)
        .expect("Something went wrong opening the file in append mode");

    // write the contents to the file opened in append mode
    // this write will take byte u8 values only, not string; thats why we need to convert to string to bytes using .as_ref()
    opened_file_to_append
        .write(contents.as_ref())
        .expect("Something went wrong writing to the file");

    println!("Contents appended to file: {:?} with Length: {}", file_to_append,contents.len());
}

// method to take user input as string as long as needed
fn get_user_input_as_string() -> String {
    let mut multi_lines = Vec::new();
    println!("Enter Texts (-1 to exit): ");
    loop{
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim()== "-1"{
            break;
        }
        multi_lines.push(input);
    }
    multi_lines.join("")
}