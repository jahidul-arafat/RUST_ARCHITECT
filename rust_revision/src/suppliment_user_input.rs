use std::io;

// Method to take input from user
fn take_input() -> Result<i32, String> {
    // Define a string buffer
    let mut input = String::new();

    println!("Do you want to enter a number (n) or a string (s)?");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim the input and check the choice
    match input.trim() {
        "n" => {
            let mut number_input = String::new();
            println!("Enter a number: ");
            io::stdin().read_line(&mut number_input).expect("Failed to read input");
            let number_input = number_input.trim().parse::<i32>();
            match number_input {
                Ok(number) => Ok(number),
                Err(_) => Err(String::from("Invalid number")),
            }
        }
        "s" => {
            let mut string_input = String::new();
            println!("Enter a string: ");
            io::stdin().read_line(&mut string_input).expect("Failed to read input");
            Ok(string_input.trim().to_string())
        }
        _ => Err(String::from("Invalid choice")),
    }
}

pub(crate) fn supplementary() {
    match take_input() {
        Ok(value) => {
            if let value_int = value.as_i32() {
                println!("You entered a number: {}", value_int);
            } else if let value_str = value.as_str() {
                println!("You entered a string: {}", value_str);
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
