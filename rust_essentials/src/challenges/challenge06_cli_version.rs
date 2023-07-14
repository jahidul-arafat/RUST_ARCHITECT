use std::{env, fs};
pub fn challenge() {
    // make sure at least 2 arguments are passed
    if env::args().len()< 3 {
        println!("Atleast 2 arguments are required");
        return;
    }

    // get the first command line argument
    let file_path = env::args().nth(1).unwrap();

    // get the second command line argument
    let search_name = env::args().nth(2).unwrap();

    // read the contents from the file_path
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    // iterate over each line in the file and check if the line contains the search_name
    for line in contents.lines() {
        if line.contains(&search_name) {
            println!("Found: {}", search_name);
            return;
        }
    }
    println!("{} Not found", search_name);
    // if the search string is not in the file, then append it to the end of the file
    println!("{} is not in the file",user_input);
    // append this to the end of the file
    let mut file = fs::OpenOptions::new().append(true).open(file_path).expect("Something went wrong opening the file");
    file.write(format!("\n{}",user_input).as_bytes()).expect("Failed to write to the file");

}