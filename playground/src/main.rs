// use std::io;

fn main() {
    // Basic loop

    // let mut counter = 0;
    // loop {
    //     println!("counter value is: {}",counter);
    //     // counter+=1; // works
    //     counter = counter+1; // also works
    //     if counter>11 {
    //         break;
    //     }
    // }

    // Loop returning value
    
    // let mut counter = 0;
    // let loop_value = loop {
    //     counter+=1; 
    //     if counter>11 {
    //         break counter; // returns a value after the loop is complete
    //     }
    // };
    // println!("This is the loop_value after loop is done: {}",loop_value);

    // For loop
    
    // let counter = [1,2,3,4,5,6,7,8,9,10];
    // for num in counter {
    //     println!("Num: {}",num);
    // }

    // for loop array mutation

    // let counter = [1,2,3,4,5,6,7,8,9,10];
    // for num in counter {
    //     // num *= 10; doesn't work
    // }

    // for num in counter {
    //     println!("Num: {}",num);
    // }

    // While loop
    let limit = 10;
    let mut counter = 0;    
    while counter <= limit {
        println!("Hi World {}",counter);
        counter+=1;
    }
}
