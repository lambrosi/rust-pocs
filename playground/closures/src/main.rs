use std::thread;
use std::time::Duration;

fn main() {

}

fn expensive_calculation(amount: u32) -> u32 {
    thread::sleep(Duration::from_secs(2));
    amount
}
