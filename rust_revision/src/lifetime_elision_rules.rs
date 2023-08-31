// The explicit lifetime annotations establish a clear relationship between the lifetimes of the input and return values,
// preventing potential issues with dangling references and helping the Rust borrow checker ensure memory safety.
pub fn simulation(){
    println!("Hello, world!");
    let message = String::from("Hello, world!");
    //let first_word = get_first_word_using_vector(&message);
    let first_word = get_first_word_using_bytes(&message);
    println!("{}", first_word);

    let msg1 = String::from("First Message");
    let msg2 = String::from("Second Message");
    let best_length_message = get_long_strings(&msg1, &msg2);
    println!("{}", best_length_message);

    let num1= 10;
    let num2= 20;
    let output = get_larger_number(num1, num2);
    println!("{}", output);
}

// method to get the first world of a String
// if in the old days, this RUST method signature would look like below; explicitly specifying the lifetime annotation
// fn get_first_word_using_vector<'a>(message: &'a str) -> &'a str
fn get_first_word_using_vector(message: &str) -> &str {
    let words: Vec<_> = message.split_whitespace().collect();
    words.first().unwrap()
}

fn get_first_word_using_bytes(message: &str)-> &str{
    let msg_bytes = message.as_bytes();
    for (i, &byte) in msg_bytes.iter().enumerate() {
        if byte == b' '{
            return &message[0..i];
        }
    }
    message // means returning the full string as it is, no space in-between founds
}

// See , this cant be done without explicit lifetime annotations
// because compiler doesnt know the lifetime of the two borrowed variable 'msg1' and 'msg2' until the Runtime
// So, we have to specify the lifetime of the two borrowed variables
fn get_long_strings<'a>(msg1: &'a str, msg2: &'a str) -> &'a str {
    if msg1.len() > msg2.len() {
        return msg1;
    }
    msg2
}

// See, no lifetime annotations needed here
// lifetime annotations are only required if you are borrowing 1+ variables at the same time and passing as a parameter into a function
fn get_larger_number(n1: i32, n2: i32) -> i32{
    if n1 > n2 {
        n1
    } else {
        n2
    }
}

