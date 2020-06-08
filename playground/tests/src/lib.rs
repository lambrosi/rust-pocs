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
    use crate::{sum, validate};

    #[test]
    fn sum_test() {
        assert_eq!(sum(2, 3), 5);
    }

    #[test]
    fn validate_valid_text_test() {
        let text = "Current year is 2020";
        assert_eq!(validate(text), text);
    }
}
