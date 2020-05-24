fn main() {
    //The if let syntax lets you combine if and let into a less verbose way
    // to handle values that match one pattern while ignoring the rest

    // Example using match (more verbose)
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // Example using if let
    if let Some(3) = some_u8_value {
        println!("three");
    }

}
