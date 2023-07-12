pub(crate) fn challenge(){
    let a = 13;
    let b=2.3;
    let c: f32= 120.0;

    // calculate the average of a,b and c
    let average = (a as f64+b +c as f64 )/3.0;
    println!("The average is {}", average);

    assert_eq!(average, 45.1);
    println!("All tests passed!");
}