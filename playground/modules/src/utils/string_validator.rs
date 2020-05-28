const MIN_SIZE: usize = 5;

pub fn validate(s: &str) -> Result<String, (String, String)> {
    let value = String::from(s);

    if value.len() < MIN_SIZE {
        return Err(build_error_tuple(value));
    }

    return Ok(value)
}

pub fn build_error_tuple(value: String) -> (String, String) {
    ("String size is less than 5".to_string(), value)
}