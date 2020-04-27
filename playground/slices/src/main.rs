fn main() {

    // A string slice is a reference to part of a String
    let text = String::from("Hello world");
    let _hello = &text[0..5];

    let first = first_word(&text);
    println!("{}", first);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
