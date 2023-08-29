use std::{fs, io};
use std::io::Write;

pub(crate) fn challenge() {
    println!("Finding named in a list");
    let file_name = String::from("person_names.txt");
    let mut name_to_search = take_user_input("Enter a name to search for: ");
    let mut file_contents = read_from_file_as_string(&file_name);
    print_contents(&file_contents);
    let is_name_exists = is_content_exists(&mut file_contents, &name_to_search);
    println!("Name {} is {} in the list", name_to_search, is_name_exists);
    if is_name_exists {
        return; // not executing the rest of the code
    }

    let mut want_to_append = take_user_input("Do you want to append: ");
    let want_to_append = want_to_append.trim();
    match want_to_append {
        "y"|"yes" => {
            append_to_file(&file_name, &name_to_search);
            print_contents(&file_contents);
        }
        "n"|"no" => {
            println!("No changes made");
        },
        _ => {
            println!("Invalid input");
        }
    }
}

// method to append to a file
pub(crate) fn append_to_file(file_to_append: &str, string_to_append: &str) {
    println!("Appending {} to File {}", string_to_append, file_to_append);

    // open the file in append mode
    let mut opened_file_to_append = fs::OpenOptions::new()
        .append(true)
        .open(file_to_append)
        .expect("Something went wrong opening the file in append mode");

    // write the contents to the file opened in append mode
    // this write will take byte u8 values only, not string; that's why we need to convert to string to bytes using .as_ref()
    opened_file_to_append
        .write(format!("\n{}", string_to_append).as_bytes())
        .expect("Something went wrong writing to the file");

    println!("Contents appended to file: {:?} with Length: {}", file_to_append,string_to_append.len());
}

pub fn read_from_file_as_string(file_to_read: &str) -> String {
    println!("File IO Read");
    let contents = fs::read_to_string(file_to_read)
        .expect("Something went wrong reading the file");

    contents.to_lowercase()
}

pub fn is_content_exists(file_content: &str, string_to_search: &str) -> bool {
    for line in file_content.lines() {
        if line.contains(string_to_search) {
            return true;
        }
    }
    return false;
}

pub fn take_user_input(prompt:&str) -> String {
    let mut input_buffer = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read line");

    let trimmed_input = input_buffer.to_lowercase().trim().parse().unwrap();
    trimmed_input
}

pub fn print_contents(file_contents: &str) {
    file_contents
        .lines()
        .enumerate()
        .for_each(|(index, line)| {println!("{}: {}", index, line)});

}
