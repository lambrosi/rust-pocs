/// Adds one to the given number
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// # Panics
///
/// This function panics if de ```argu``` is bigger than 10
///
/// # Errors
///
/// Other documentation option "errors"
///
/// # Safety
///
/// Describe when a function is unsafe
///
pub fn add_one(x: i32) -> i32 {
    if x > 10 {
        panic!("The number must be lower than 10");
    }
    x + 1
}

// To generate the HTML doc, run:
// cargo doc
// or
// cargo doc --open