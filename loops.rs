//Loops - Used to iterate until a condition is met

pub fn run() {

    let mut count = 0;
    let mut count1 = 0;

    //Infinite Loop
    loop {
        count1 += 1;
        println!("Number {}",count1);

        if count1 == 20 {
            break;
        }
    }
/*
    // While Loop (FizzBuzz)
    while count <= 100 {
        if count  % 15 == 0 {
            println!("fizzbuzz");
        }else if count % 3 == 0 {
            println!("Fizz");
        }else if count % 5 == 0 {
            println!("Buzz");
        }else {
            println!("{}",count );
        }

        //increment 
        count += 1;
    }
*/

    // For range 
    for x in 0..100 {
        if x  % 15 == 0 {
            println!("fizzbuzz");
        }else if x % 3 == 0 {
            println!("Fizz");
        }else if x % 5 == 0 {
            println!("Buzz");
        }else {
            println!("{}",x );
        }
    }

}