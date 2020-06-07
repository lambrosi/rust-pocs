use std::env;

fn main() {

    let args: Vec<String> = get_args();
    let arg1 = &args[1];
    let arg2 = &args[2];

    let longest = longest(arg1, arg2);

    println!("Informed parameters: '{}' and '{}'\nLongest: '{}'", arg1, arg2, longest);
}

/// Here is used a lifetime paramter 'a
/// Lifetime parameters is only used with references
/// This indicates that string 'text' and 'another'
/// Lives in the same context
fn longest<'a>(text: &'a str, another: &'a str) -> &'a str {
    if text.len() > another.len() {
        return text
    }
    another
}

fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Two args must be informed.");
    }
    args
}
