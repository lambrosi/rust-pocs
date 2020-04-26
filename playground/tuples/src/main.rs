fn main() {
    //Declaring a tuple
    let tuple: (f64, u8) = (2.5, 120);
    println!("Value of tuple {:?}.", tuple);

    //Destructuring the tuple (break into pieces)
    let (x, y): (f64, u8) = tuple;
    println!("Value of x: {} and value of y: {}.", x, y);

    //Another way to break into pieces
    println!("Value of first: {} and value of second: {}.", tuple.0, tuple.1);
}
