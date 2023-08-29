use std::env;
use std::process::exit;
use challenge06::{take_user_input, is_content_exists, read_from_file_as_string, append_to_file, print_contents};
use crate::challenge06;

// cargo run "person_names.txt" "jahid"
pub(crate) fn challenge(){
    println!("Finding named in a list");
    let(file_name,name_to_search)=parse_arguments_at_cli();
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

fn parse_arguments_at_cli()->(String,String){
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Not enough arguments");
        exit(1);
    }
    (
        parse_string(&args[1].to_string()),
     parse_string(&args[2].to_string())
    )

}

fn parse_string(input_string:&str)->String{
    input_string.to_lowercase().trim().parse().unwrap()
}