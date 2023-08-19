pub(crate) fn challenge(){
    println!("Challenge 1");
    let a=13;
    let b=2.3;
    let c: f32=120.0;

    // calculate average
    let average = (a as f64+b+c as f64)/3.0;
    println!("Average: {}", average);

    assert_eq!(average, 45.2);
    println!("All tests passed!");
}
