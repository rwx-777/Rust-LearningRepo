// Vectors are resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //Reassign value
    numbers[2] = 20;

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //Pop off the last value
    numbers.pop();

    println!("{:?}",numbers ); //prints out all values in the array

    //Get single value
    println!("Single Value: {}",numbers[0] );

    //Get an Vector Length
    println!("Vector Length: {}",numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes",std::mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}",slice );

    //Get5 first two
    let secondSlice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}",secondSlice);

    //Loop thru vector values
    for x in numbers.iter() {
        println!("Number: {}",x);
    }

    //Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2; //each value will be multiplied by two
    }
    println!("Numbers Vec: {:?}",numbers);

}