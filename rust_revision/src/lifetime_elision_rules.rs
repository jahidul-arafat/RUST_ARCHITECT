// The explicit lifetime annotations establish a clear relationship between the lifetimes of the input and return values,
// preventing potential issues with dangling references and helping the Rust borrow checker ensure memory safety.
/*
3 Lifetime Elision Rules for Modern Rust
(a) (Obvious/Qualified for elision) Function with exactly one borrowed(reference)/non-borrowed(non-reference) parameter, assign it to all output lifetime without
explicit lifetime annotations.
    fn get_first_word(s: &str) -> &str {}
(b)
- Function with 1 borrowed/reference parameter and other non-borrowed/non-reference parameters
    fn get_first_word<'a>(s: &'a str, y:i32) -> &'a str {}
- Function with 1+ borrowed/reference parameters i.e. three borrowed parameters,
can have same lifetime annotations for all input and output or different lifetime annotations for each input
    fn get_longest<'a, 'b,'c>(s: &'a str, y:&'b str,z: &'c str) -> &'a str {}
    fn get_longest<'a>(s: &'a str, y:&'a str,z: &'a str) -> &'a str {}

(c) For Method in a Struct, having &self or &mut self parameter, then its lifetime will be automatically assigned to the output's lifetime
// no explicit lifetime annotations will be required, Rust will convert this automatically
(how we write)
fn set_name(&mut self, name: &str)-> &str{

(how rust convert it with lifetime elision rules)
fn set_name<'a,'b>(&'a mut self, name: &'b str) -> &'a str{

 */
pub fn simulation() {
    println!("Hello, world!");
    let message = String::from("Hello, world!");
    //let first_word = get_first_word_using_vector(&message);
    let first_word = get_first_word_using_bytes(&message);
    println!("{}", first_word);

    let msg1 = String::from("First Message");
    let msg2 = String::from("Second Message");
    let best_length_message = get_long_strings(&msg1, &msg2);
    println!("{}", best_length_message);

    let num1 = 10;
    let num2 = 20;
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

fn get_first_word_using_bytes(message: &str) -> &str {
    let msg_bytes = message.as_bytes();
    for (i, &byte) in msg_bytes.iter().enumerate() {
        if byte == b' ' {
            return &message[0..i];
        }
    }
    message // means returning the full string as it is, no space in-between founds
}

// See , this cant be done without explicit lifetime annotations
// because compiler doesnt know the lifetime of the two borrowed variable 'msg1' and 'msg2' until the Runtime
// So, we have to specify the lifetime of the two borrowed variables along with the lifetime of the return value to associate inputs with output
fn get_long_strings<'a>(msg1: &'a str, msg2: &'a str) -> &'a str {
    if msg1.len() > msg2.len() {
        return msg1;
    }
    msg2
}

// See, no lifetime annotations needed here
// lifetime annotations are only required if you are borrowing 1+ variables at the same time and passing as a parameter into a function
fn get_larger_number(n1: i32, n2: i32) -> i32 {
    if n1 > n2 {
        n1
    } else {
        n2
    }
}

