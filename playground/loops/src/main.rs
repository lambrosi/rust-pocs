fn main() {
    //Rust has three kinds of loops: loop, while, and for.
    
    // [LOOP] - execute a block of code over and over again forever or until you explicitly tell it to stop
    //simple usage
    loop {
        println!("again");
        break;
    }

    //Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // [WHILE]
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // [FOR]
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("Value from array is {}.", element);
    }

    // Another for sample
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
