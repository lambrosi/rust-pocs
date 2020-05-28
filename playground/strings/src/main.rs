fn main() {

    // This are equivalent for creating a String
    let t1 = "teste".to_string();
    let t2 = String::from("teste");

    // Updating string
    // push_string => receive a piece of text
    // push        => receive only one character
    let mut text = String::from("Hello");
    text.push_str(", World");
    text.push('!');
    println!("New String: {}", text);      // "Hello, World!"

    // Concatenation with '+' and format! macro
    // With '+' only is possible add one &str to a String; can't add two String
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; //s1 has been moved here and can no longer be used

    let s4 = format!("{},{}!", "Hello", "World");

    // Slicing strings
    let s5 = "teste542345";
    let s6 = &s5[0..4]; //gets the first 4 bytes of 's5'

    // Iterate over strings
    for s in s6.chars() {
        println!("{}", s);
    }

    for b in s6.bytes() {
        println!("{}", b);
    }
}
