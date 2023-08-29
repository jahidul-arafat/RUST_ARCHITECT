// &str -> string slice data type
// length --> number of bytes, not characters
// basic character--> 1 byte
// &--> borrowed reference
pub(crate) fn simulation() {
    println!("Hello, world!");

    let mut msg = String::from("Greetings from Earth!");
    println!("{}", msg);

    // let last_word = &mut msg;
    // *last_word =String::from("Test");
    // println!("last word: {}",last_word);
    // println!("Message: {}",msg)

    let last_word = &msg[15..];
    println!("last word: {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("Planets: {:?}", planets);
    let inner_planets = &planets[..4];
    println!("Inner Planets: {:?}", inner_planets);

    let mut first_word = get_first_word_by_bytes(&msg[10..]);
    println!("First word: {}", first_word);

    first_word = get_first_word(&msg);
    println!("First word: {}", first_word);

    first_word = get_first_word_by_bytes(&msg); // example of deref coercion using String reference as slice, but opposite is not true
    println!("First word: {}", first_word);

    //first_word = get_first_word(&msg[10..]); // opposite not true
    //println!("First word: {}", first_word);
}

// &String != &str
/*
&String data has both length and capacity point to stack of origin which then to Heap at ownership
&str data has only length directly pointing to Heap at borrowing
 */

fn get_first_word_by_bytes(msg: &str) -> &str {
    println!("str length: {}", msg.len());
    //println!("str capacity: {}", msg.capacity()); // slice data has no capacity defined
    let bytes = msg.as_bytes();
    println!("Bytes: {:?}", bytes);

    for (index, item) in bytes.iter().enumerate() {
        if *item == b' ' {
            return &msg[0..index];
        }
    }
    &msg
}

fn get_first_word(msg: &str) -> &str {
    println!("String Length: {}", msg.len());
    //println!("String Capacity: {}", msg.capacity());
    for (index, item) in msg.chars().enumerate() {
        if item == ' ' {
            return &msg[0..index];
        }
    }
    &msg
}