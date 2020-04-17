//Primitive string = Immutable fixed length string somehwere in memory
// String = growable, heap allocated data structure - use when you need to modify your own string data 

pub fn run() {

    let greeting = "Hello there";
    let mut hello = String::from("Hello");

    //Get length of string
    println!("Length: {}",hello.len());

    //Add a char to a string
    hello.push('w');

    //add a string to a string
    hello.push_str(" owo");
    //does not work with greeting has to be string type

    //get number of bytes it can store
    println!("Capacity: {}",hello.capacity());

    //Check if string is empty
    println!("Is Empty: {}",hello.is_empty());

    //Contains
    println!("Contains 'owo' {}",hello.contains("owo"));

    //Replace 
    println!("Replace: {}", hello.replace("owo","uwu"));

    //Loop thru String by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}",s);

    //Assertion testing (what is equal to what ?)
    assert_eq!(2, s.len());
    assert_eq!(10,s.capacity());

    //println!("{:?}",(greeting,hello ));


}