use std::thread;
use std::time::Duration;

fn main() {
    let value = 10;

    print_result(value, true);
}

fn print_result(value: u32, standard: bool) {
    let expensive_closure = |number| {
        expensive_calculation(number)
    };

    // Or without {}, because this are a simple expression
    // let exp = |num| expensive_calculation(num);

    if value > 5 {
        println!("First opt: {}", expensive_closure(value));
        println!("Second opt: {}", expensive_closure(value));
    } else {
        if standard {
            println!("Standard...");
        } else {
            println!("Customized: {}", expensive_closure(value));
        }
    }
}

fn expensive_calculation(amount: u32) -> u32 {
    println!("Perfoming calculation...");
    thread::sleep(Duration::from_secs(2));
    amount
}
