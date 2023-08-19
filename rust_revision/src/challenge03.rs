pub(crate) fn challenge(){
    println!("Min Max Using Ownership Model ");
    let mut numbers=[2,1,3,4,5,6];
    numbers.sort();
    println!("{:?}",numbers);
    let (max,min,total) = maxmin(&numbers);
    println!("max:{} min:{}",max,min);
    println!("Average: {}",total/numbers.len() as f64);
}

// method to find the max value in an array
fn maxmin(numbers: &[i32]) -> (i32,i32,f64) {
    let mut max = numbers[0];
    let mut min = numbers[0];
    let mut total=0.0;
    for item in numbers{
        total+=*item as f64;
        if *item > max{
            max = *item;
        } else if *item<=min{
            min = *item;
        }
    }
    (max,min,total)
}