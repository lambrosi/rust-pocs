fn sum(a: u32, b: u32) -> u32 {
    a + b
}

fn validate(text: &str) -> &str {
    if text.contains("2020") {
        return text
    }
    panic!("Invalid text: {}", text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        assert_eq!(sum(2, 3), 5);
    }

    #[test]
    fn validate_valid_text_test() {
        let text = "Current year is 2020";
        assert_eq!(validate(text), text);
    }

    #[test]
    #[should_panic]
    fn validate_invalid_text_test() {
        validate("invalid");
    }

    // Should panic with validation on panic message
    #[test]
    #[should_panic(expected = "Invalid text:")]
    fn validate_invalid_text_test2() {
        validate("invalid");
    }
}

// To ignore a specific test, use:
// #[ignore]

// To run only ignored tests, use:
// 'cargo test -- --ignored'

// Running 'cargo test --help' displays the options you can use with 'cargo test'
// Running 'cargo test -- --help' displays the options you can use after the separator '--'

// Tests, by default, run in parallel
// To run 1:1, use this command:
// 'cargo test -- --test-threads=1'

// Run one specific
// Example: run only tem 'sum_test' in this file
// 'cargo test sum_test'

// Filter to run multiple tests
// Example: run all tests with 'validate' in the name
// 'cargo test validate'
