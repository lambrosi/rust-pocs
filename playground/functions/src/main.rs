fn main() {
    print(250);
    print(five());

    let value = plus_one(5);
    print(value);
}

//Function without return
fn print(value: u8) {
    println!("The value is {}.", value);
}

//Function with return (the 'return' keyword is optional since functions return the last expression implicitly)
fn five() -> u8 {
    5
}

//Function with return receiving argument
fn plus_one(value: u8) -> u8 {
    value + 1
}
