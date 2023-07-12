// RUST need to know the data type of every variable at Compile time.
pub(crate) fn simulation(){
    println!("Program Flow Control Simulation");
    let x=3;
    if x==3 && x>2{
        println!("x is 3 and greater than 2");
    } else {
        println!("x is not 3, Second expression will not be evaluated");
    }

    let make_x_odd = true;
    let x = if make_x_odd { 1 } else { 2 }; // shortcut if-else statement
    println!("x is {}", x);

    // for loop to print numbers from 1 to 10
    for i in 1..11 {
        println!("i is {}", i);
    }

    let mut count=0;

    let result =loop {
        count += 1;
        println!("count is {}", count);
        if count == 20 {
            break count*10; // break the loop when count reaches 20 and return 20*10 // break statement in loop can return a value
        }

    };
    println!("result is {}", result);

    // while loop to print numbers from 1 to 10
    let mut count=0;
    while count < 11 {
        if count==5 {
            break; // to exit prematurely // break the loop when count reaches 5 and return 5*1 // break statement in While Loop cant return value
        }
        count += 1;
        println!("count is {}", count);
    };

    /*
             numbers (Vec<i32>)
         +---+---+---+---+---+
Index -> | 1 | 2 | 3 | 4 | 5 |
         +---+---+---+---+---+
           ^
           |
     borrowed reference (&numbers)

     The borrowed reference &numbers allows the loop to access the elements of the vector without transferring ownership.
     It's a way to work with the data without changing its ownership or consuming it.

     */

    // create an array of letters a to f
    let letters = ['h', 'e', 'l', 'l', 'o'];

    // add a new element to the array

    // iterate over the array and print each letter
    // for letter in letters{ // iterating the array by reference &[1,2,3] or [1,2,3].iter()
    //     println!("By Value letter is {}", letter);
    // }

    // iterate over the array and print each letter and get their index
    // enumeration dont return us the actual value, but a reference to the value
    for (index, &letter) in letters.iter().enumerate(){ // iterating the array be reference &[1,2,3] or [1,2,3].iter()
        println!("Index {1} is {0}", letter, index);
        if letter=='l' {
            break;
        }
    }

    // let mut count=0;
    // while count < letters.len() {
    //     println!("letter is {}", letters[count]);
    //     count+=1;
    // }

    for number in 1..11 {
        println!("number is {}", number);
    }

    // create a 2D array of numbers
    let mut numbers = [
        [10, 2, 3],
        [4, 50, 6],
        [7, 8, 90]];
    println!("numbers is {:?}", numbers);
    // iterate over the array and print each number
    for row in numbers {
        for mut number in row {
            number+=10;
            print!("{}\t", number);
        }
        println!();
    }




}