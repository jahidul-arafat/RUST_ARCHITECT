pub(crate) fn compound_datatype(){
    // define an array of 5 characters
    let mut letter_array: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    letter_array[0] = 'z';
    println!("First letter: {}", letter_array[0]);
    println!("Second letter: {}", letter_array[1]);

    // create an array of integers
    let integer_array: [i32; 5]; // [DataType, NumberOfElements] // help the compiler to estimate the memory space required

    // initialize the array with all zeros
    let mut integer_array: [i32; 5] = [0; 5]; // contains five copies of 0 in the array

    // get the length of the array
    let length = integer_array.len(); // vary with datatype; for i32, usize=4bytes; for i64, usize=8bytes
    println!("Length of the array: {}", length);

    println!("First integer: {}", integer_array[0]); // will through a Bug as array is not initialized
    println!("last integer: {}", integer_array[length - 1]);

    // define a 2D array
    // By using {:?} instead of {}, the code will print the value of array_2d[0]
    // in a format that is more helpful for debugging and understanding the structure of the value.
    let mut array_2d = [
                                                [1,2,3],
                                                [4,5,6],
                                                [7,8,9]];
    println!("First row: {:?}", array_2d[0]);
    println!("Second row: {:?}", array_2d[1]);
    println!("Third row: {:?}", array_2d[2]);
    println!("First Element: {:?}", array_2d[0][0]);

    // create a 3D array of 5 by 20 by 100 integer elements
    let mut array_3d = [[[0; 100]; 20]; 5];

    // create a tuple
    let mut tuple = (1, 2.2, 'a');
    println!("tuple: {:?}", tuple);
    println!("tuple.0: {}", tuple.0);
    tuple.1+=2 as f32;
    println!("tuple.1: {}", tuple.1);
    println!("tuple.2: {}", tuple.2);

    // print the length of the tuple
    //println!("tuple.len(): {}", tuple.len);

    // destructure the tuple
    let (x, y, z) = tuple;
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);



}