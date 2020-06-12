use std::thread;
use std::time::Duration;

fn main() {
    let expensive_closure = |number| {
        expensive_calculation(number)
    };

    // Or without {}, because this are a simple expression
    let exp = |num| expensive_calculation(num);

    println!("Result: {}", expensive_closure(15));
}

fn expensive_calculation(amount: u32) -> u32 {
    println!("Perfoming calculation...");
    thread::sleep(Duration::from_secs(2));
    amount
}
