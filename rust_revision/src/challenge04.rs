pub(crate) fn challenge(){
    println!("Challenge 4- Removing leading and trailing spaces from a String");
    let mut s = String::from(" S Hello World   ...              ");
    let output= remove_space(&s);
    println!("Before trimming: {}", s.len());
    println!("After trimming: {}, length: {}", output, output.len());
}

fn remove_space(input:&str)->&str{
    // convert the string into bytes
    let input_bytes = input.as_bytes();
    println!("input_bytes: {:?}", input_bytes);

    let mut start_index=0;
    let mut end_index=input_bytes.len()-1;

    // find the index of the first non-whitespace character
    while start_index < end_index && input_bytes[start_index].is_ascii_whitespace() {
        start_index+=1;
    }

    // find the index of the last non-whitespace character
    while start_index < end_index && input_bytes[end_index].is_ascii_whitespace() {
        end_index-=1;
    }

    &input[start_index..end_index+1]
}