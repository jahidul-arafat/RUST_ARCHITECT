pub fn simulation(){
    println!("Data Race (Simplified) Simulation");

}


/*
// Data Race Scenario-1
fn error_scenario_cannot_borrow_data_as_immutable_because_it_is_also_borrowed_as_mutable(){
    let mut data = vec ! [1, 2, 3];
    let ref1 = &mut data;
    let ref2 = & data; // error: cannot borrow `data` as mutable, as it is also borrowed as immutable

    ref1.push(4);
    println ! ("Ref1 data: {:?}", data);

    println ! ("Ref2 data: {:?}", ref2);
}

// Data Race Scenario-2
fn error_scenario_cant_borrow_data_as_mutable_more_than_once_at_a_time(){
    let mut data = vec ! [1, 2, 3];
    let ref1 = &mut data;
    let ref2 = &mut data; // error: cannot borrow `data` as mutable, as it is also borrowed as immutable

    ref1.push(4);
    println ! ("Ref1 data: {:?}", data);

    println ! ("Ref2 data: {:?}", ref2);

}
*/

