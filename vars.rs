//Varibles hold primitive data or reference to data
//Varibles are imutable by default

pub fn run() {
    let name = "Elliot";
    let age = 27; //these values cannot be changed 
    let mut mutable_age = 27; //now the varibale is mutable we can chnage it in the code anytime
    println!("My name is {} and I am {}",name,age);
    mutable_age=38;
    println!("My name is {} and I am {}",name, mutable_age );

    //Assign multiple varibles at once
    let (myname, my_age ) = ("Darleen", 29);
    println!("Name:{} Age:{}",myname, my_age);
}