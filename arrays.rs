//Fixed list of elements with the same data types

pub fn run() {
    //When declaring an Array the datatype(i32) and Length(5) have to be exact !
    let numbers: [i32; 5] = [1,2,3,4,5]; //Array of five elements

    println!("{:?}",numbers ); //prints out all values in the array

    //Get single value
    println!("Single Value: {}",numbers[0] );

    //Get an Array Length
    println!("Array Length: {}",numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes",std::mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}",slice );

    //Get5 first two
    let secondSlice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}",secondSlice);

}