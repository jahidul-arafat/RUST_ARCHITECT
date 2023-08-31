pub fn simulation(){
    println!("How Lifetime annotation could Prevent use-after-free and double-free vulnerabilities?");
    use_after_free_vulnerability();
    println!("");
    double_free_vulnerability();


}

fn use_after_free_vulnerability(){
    let mut data = vec![1,2,3]; // at stack
    let borrowed_data = &data[0]; // borrowing, ownership not transferred
    //println!("Borrowed data: {}", borrowed_data);
    data.clear(); // deallocation of memory
    //println!("Borrowed data: {}", borrowed_data); // Error// still trying to use the borrowed data, after original data has been freed
    // RUST prevents this during compile time
    // This prevent use-after-free vulnerabilities
    println!("Original data: {:?}", data);

}

fn double_free_vulnerability(){
    let mut data = Box::new(42);
    let borrowed_data = data; // ownership is transferred // data is already freed

    //drop(data); // double deallocation of memory
    println!("Borrowed data: {}", borrowed_data);
    //println!("Original data: {:?}", data);
}