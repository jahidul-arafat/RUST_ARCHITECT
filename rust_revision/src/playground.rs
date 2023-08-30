pub fn playground(){
    let mut data = String::from("Test Data");
    let result = return_str(&mut data);
    println!("Result: {}", result);
}

fn return_str(input: &mut str) -> &str {
    let x = "JA";
    x
}