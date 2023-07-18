/* Reserved lifetime name is 'static, means data is available for entire duration of the program;
    - means data never get dropped
    Example:
    let s = "Welcome to JA RUST Sim Labs"; // here s is in a static lifetime s:&'static str = "Welcome to JA RUST Sim Labs";

 */
// Struct and Lifetime management simulation, when Struct doesnt own a String, instead it borrows
// define a struct named 'Shuttle'
#[derive(Debug)]
struct Shuttle<'a>{
    name: &'a str // see, the Struct doesn't own the String, instead it borrowed it
    // that's why there could be a probability that the Struct might get out of scope
    // to avoid this, we have to explicitly specify the lifetime of the struct
}

// implement constructor, getters and setters
impl<'a,'b> Shuttle<'a>{
    fn new(name: &str) -> Shuttle{
        Shuttle{
            name
        }
    }
    fn get_name(&self) -> &str{
        self.name
    }
    fn set_name(&mut self, name: &'a str){
        self.name = name;
    }
    fn send_transmission(&'a self, msg:&'b str)-> &'b str{
        println!("Sending message: {}", msg);
        //self.name // no explicit lifetime annotation required, due to Lifetime elission rule 3
        msg // but, here we are returning the msg, which requires an explicit lifetime annotation
    }
}

pub fn simulation(){
    println!("Struct Lifetime Management Simulation");

    // create a struct named 'Shuttle'
    let mut shuttle = Shuttle::new("Shuttle");
    shuttle.set_name("Shuttle 2");
    println!("Shuttle Name: {}", shuttle.get_name());

    println!("Shuttle: {:?}", shuttle);
    let sender = shuttle.send_transmission("Hello World!");
    println!("Shuttle: {:?}", shuttle);
    println!("Sender is: {}", sender);

    let s = "Welcome to JA RUST Sim Labs";