pub fn run() {
    // print to console
    println!("Hello from the print file");

    //inserts the int into a string literal
    //you cannot print ints
    //print!("{}",1) 

    //lets have multiple placeholders
    println!("{} is from {}","Elliot","NYC");

    //Positional Arguments
    //uses indexes for placeholders 0 stands for elliot and so on
    println!("{0} is from {1} and {0} likes to {2}","Elliot","NYC","code");

    //Named arguments
    println!("{name} likes to play {activity}",name="Elliot",activity="ctfs");

    //Placeholder traits
    //This is pretty cool we type a number and it will output it into binary,hex & octal
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

    println!("Binary: {:b} Hex: {:x} Octal: {:o}",192,192,192);

    //placeholder for debug traits
    //Tuple
    println!("{:?}", (12,true,"hello" ));
}