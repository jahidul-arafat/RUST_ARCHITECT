use std::mem;

pub fn simulation(){
    println!("Hello, world!");
    // Box<T> data type-> Stores data in the Heap, even if T is a datatype storing data at Stack
    // Box<T> is smart pointer
    smart_pointer();
}

#[derive(Debug)]
struct Shuttle{
    name: String,
    crew_size: u8,
    propellant: f64
}

/*
Why Box<T> is used?
- to store a tyoe whose size cannot be known at compile time
- Example: Recursive Type. I.e. a Struct, having a field of another Struct, which has a field of another Struct, ....
- You may not know how many recursion call at compile time
- To avoid copying large amount of stack data and moving those to Heap by transferring ownership
 */
fn smart_pointer(){
    // create an instance of Shuttle
    // stack pointing the String at Heap
    // shuttle is in Stack
    let shuttle = Shuttle{
        name: String::from("Shuttle-1"), // in heap // 24 bytes
        crew_size: 10, // in stack
        propellant: 100.0 // in stack
    };

    println!("Shuttle size on Stack: {} bytes", mem::size_of_val(&shuttle)); // 40bytes
    println!("Original {:#?}", shuttle);

    // move this struct into a Heap by using Box
    // means all the shuttle data i.e. name, crew_size, propellant will be in the Heap where name in the heap pointing to a String in the Heap
    let boxed_shuttle = Box::new(shuttle); // transfer ownership of shuttle into boxed_shuttle; use &shuttle if to borrow
    println!("Boxed {:#?}", boxed_shuttle);

    println!("Boxed Shuttle (Pointer) size on Stack: {} bytes", mem::size_of_val(&boxed_shuttle)); // 8 bytes pointer at the stack
    // but how much this box data occupied in the Heap?
    // Solution: use derefencing operator to get the value of boxed_shuttle
    // means, dereferencing the boxed_shuttle pointer which is in stack pointing to a heap
    // * -> dereferencing the pointer to get the actual value
    println!("Boxed Shuttle (Actual Value) size on Heap: {} bytes", mem::size_of_val(&*boxed_shuttle)); // deref, get the actual value


    //println!("Shuttle size on Stack: {} bytes", mem::size_of_val(&shuttle)); // shuttle no longer exists as moved to Box

    // move that boxed data from Heap to Stack
    let unboxed_shuttle = *boxed_shuttle; // transfer the actual boxed_shuttle values from Heap to Stack again// ownership is transferred to unboxed_shuttle
    println!("Unboxed {:#?}", unboxed_shuttle);
    println!("Unboxed Shuttle (Actual Value) size on Stack: {} bytes", mem::size_of_val(&unboxed_shuttle));

    // check if boxed_shuttle is still in the Heap or not or does it still own it
    //println!("Is boxed_shuttle still in the Heap: {:?}", boxed_shuttle); // will trigger error// ownership already transferred to unbox_shuttle. So, boxed_shuttle ownes nothing now

}
