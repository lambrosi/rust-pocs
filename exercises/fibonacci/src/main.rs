fn main() {
    println!("{:?}", get_fibonacci_sequence(10));
    println!("The 9th element of sequence (disregarding the zero) is: {}", calculate_fibonacci_at(9));
}

fn get_fibonacci_sequence(amount: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    vec.push(0);
    vec.push(1);

    for _number in 2..amount {
        let last_index = vec.len();
        vec.push(vec[last_index - 1] + vec[last_index - 2]);
    }
    return vec;
}

fn calculate_fibonacci_at(number: u32) -> u32 {
    if number <= 1 {
        return number
    }

    return calculate_fibonacci_at(number - 1) + calculate_fibonacci_at(number - 2)
}
