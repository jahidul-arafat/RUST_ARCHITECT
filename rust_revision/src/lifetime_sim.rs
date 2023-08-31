pub fn simulation(){
    println!("Lifetime Simulation!");
    //borrow_checking_lifetime_malfunction();
    //borrow_checking_lifetime_correction();
    best_fuel_same_lifetime_simulation();
}

fn borrow_checking_lifetime_malfunction(){
    // 'a
    let propellant;
    {
        // 'b
        let rp1= String::from("RP-1");
        propellant =&rp1; // borrowing, not passing ownership
        println!("Propellant: {}", propellant);
        println!("RP Value: {}", rp1); // rp1 still have the value
    }
    //println!("Propellant: {}", propellant); // Error // lifetime: out of scope
} // 'a

fn borrow_checking_lifetime_correction(){
    // 'a
    let propellant;
    let rp1= String::from("RP-1");
    {
        // 'b
        propellant =&rp1; // borrowing, not passing ownership
        println!("RP Value: {}", rp1); // rp1 still have the value
    } // 'b
    println!("Propellant: {}", propellant); // Error // lifetime: out of scope
} // 'a

//fn best_fuel_lifetime_specifier_error(x:&str, y:&str) -> &str{
// 'a -> lifetime annotation, defined explicitly here
// tells compiler how long the lifetime of the both variable would be
// if variable x and y has different lifetime, compiler would be in 'restrictive' mode, picking the least lifetime of the both
// WHY we need these annotations? WHY RUST cant detect its own and decide the parameter's lifetime?
fn best_fuel_lifetime_annotation<'a,'b>(x:&'a str, y:&'b str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        x
        //y
    }
}

fn best_fuel_same_lifetime_simulation(){
    let fuel1 = String::from("RP-1");
    let fuel2 = String::from("LPG");
    let best_fuel = best_fuel_lifetime_annotation(&fuel1, &fuel2);
    println!("Best fuel: {}", best_fuel);
}

fn best_fuel_shorter_lifetime_simulation(){
    let best_fuel;
    let fuel1 = String::from("RP-1");
    {
        let fuel2 = String::from("LPG");
        best_fuel = best_fuel_lifetime_annotation(&fuel1, &fuel2); // fuel2 doesnt live long enough
        //println!("Best fuel: {}", best_fuel);
    }
    println!("Best fuel: {}", best_fuel);
}