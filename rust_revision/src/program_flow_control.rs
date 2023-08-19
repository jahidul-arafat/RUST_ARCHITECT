pub(crate) fn simulation(){
    println!("Program Flow Simulation");

    let x=3;
    if x==3 && x>2{
        println!("x is 3 and greater than 2");
    }else {
        println!("x is not 3 or greater than 2");
    }

    let make_x_odd=true;
    let output = if make_x_odd{1} else {0};
    println!("output is {}",output);

    for i in 0..11{
        println!("i is {}",i);
    }

    let mut count=0;
    let count_return =loop {
        count+=1;
        println!("count is {}",count);
        if count==10{
            break count*10; //break and return count*10
        }
    };
    println!("count_return is {}",count_return);

    println!("While Statement Simulation");
    let mut count=0;
    while count<11{

        count+=1;
        println!("count is {}",count);
        if count==5{
            break;
        }
    };

    let mut letters = ['h','e','l','l','o'];
    let mut borrowed_letters= &mut letters; // mutable binding
    borrowed_letters[0]='w';
    println!("borrowed_letters is {:?}",borrowed_letters);
    println!("letters is {:?}",letters);

    let mut ownership_letters = letters;
    ownership_letters[0]='y';
    println!("ownership_letters is {:?}",ownership_letters);
    println!("letters is {:?}",letters);


    let mut s = String::from("hello");
    let mut s1_borrowed = &mut s;
    s1_borrowed.push_str(" world!");
    println!("s1_borrowed is {:?}",s1_borrowed);
    println!("s is {:?}",s);

    let mut s1_ownership = s;
    s1_ownership.push_str(" world!");
    println!("s1_ownership is {:?}",s1_ownership);
    //println!("s is {:?}",s); // as ownership is transferred to s1_ownership and s1 holds nothing // this is the case in String, not for Array

    // create a 2D array
    let mut number2d = [
        [1,2,3],
        [4,5,6],
        [7,8,9]
    ];
    println!("number2D is {:?}",number2d);

    // update each element of the array by 10
    for row in &mut number2d{
        for element in row{
            *element+=10; // The * operator is used to dereference the reference and access the actual value in the array, and then the value is incremented by 10.
        }
    }
    println!("number2D is {:?}",number2d);

    let mut numbers =[1,2,3,4,5,6,7,8,9];
    println!("numbers is {:?}",numbers);
    for num in &mut numbers{
        *num+=10; // dereference the reference and access the actual value in the array, and then the value is incremented
    }
    println!("numbers is {:?}",numbers);

    let mut numbers_borrowed = numbers.clone();// deep copy // =numbers;// shallow copy //array implements copy traits //but string doesnt
    // numbers_borrowed[0]=100;
    for mut num in &mut numbers_borrowed{
        *num+=1000; // dereference the reference and access the actual value in the array, and then the value is incremented
    }
    println!("numbers_borrowed is {:?}",numbers_borrowed);
    println!("numbers is {:?}",numbers);


}

