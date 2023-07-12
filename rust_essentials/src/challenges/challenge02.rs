pub(crate) fn challenge(){
    let temp_fahrenheit = celsius_to_fahrenheit(23.0);
    println!("{:?} (Celsius, Fahrenheit)", temp_fahrenheit);

    assert_eq!(temp_fahrenheit, (23.0,73.4));
    println!("All tests passed!");
}

// method to convert temperature from celsius to fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> (f64,f64) {
    (celsius,celsius * 9.0 / 5.0 + 32.0)
}