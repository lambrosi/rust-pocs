mod utils;
use utils::string_validator;

fn main() {
    match string_validator::validate("tac") {
        Ok(s) => println!("String is: {}", s),
        Err((error, value)) => println!("Invalid string '{}'. Detail: {}", value, error)
    }
}
