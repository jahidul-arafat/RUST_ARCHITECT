pub(crate) fn simulation() {
    println!("Compound Datatype simulation\nArray, MultiD Array, Tuples!!!");

    // create an array of letters
    let mut letters = ["a", "b", "c", "d", "e"];
    println!("Array of letters: {:?}", letters); // debug format {:?}
    println!("First letter: {}", letters[0]);
    println!("Last letter: {}", letters[letters.len() - 1]);
    println!("Length of letters: {}", letters.len());


    println!("Array of letters: {:?}", letters);

    // create an array of integers
    let int_array: [i32; 5];
    int_array = [0; 5];
    println!("Array of integers: {:?}", int_array);

    let mut  array_2d =[
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    println!("Array 2D: {:?}", array_2d);
    println!("Array 2D[0]: {:?}", array_2d[0]);

    let mut array_3d =[[[0;100];20];5];
    println!("Array 3D: {:?}", array_3d);

    // create a tuple
    let mut tuple = (1,2.2,'a');
    println!("Tuple: {:?}", tuple);
    println!("First element of tuple: {}", tuple.0);
    println!("Second element of tuple: {}", tuple.1);

    // print length of tuple
    // tuple.len()  not available in Rust

    // destructure the tuple
    let (x,y,z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);

    




}
