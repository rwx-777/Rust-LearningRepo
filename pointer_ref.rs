// Reference Pointers - point to a resource in Memory

pub fn run() {
    //Primitive Array
    let array1 = [1,2,3];
    let array2 = array1;

    println!("Values: {:?}",(array1,array2));

    //with non primitives if you assign another variables to a piece of data the first var will no longer
    // hold that value. Youll need to use a reference (&) to point to the resource

    let vec1 = vec![1,2,3];
    let vect2 = &vec1; //& states that we want it pointing to a reference

    println!("Values: {:?}",(&vec1,vect2));


}