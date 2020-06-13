use std::thread;
use std::time::Duration;

fn main() {
    let value = 10;

    print_result(value, true);
}

fn print_result(intensity: u32, standard: bool) {
    // If the expression have multiple lines, use {}
    // let expensive_closure = |number| {
    //     let n = number 2;
    //     n * 4
    // };

    let mut expensive_closure = Cacher::new(|num| expensive_calculation(num));

    if intensity > 5 {
        println!("First opt: {}", expensive_closure.value(intensity));
        println!("Second opt: {}", expensive_closure.value(intensity));
    } else {
        if standard {
            println!("Standard...");
        } else {
            println!("Customized: {}", expensive_closure.value(intensity));
        }
    }
}

fn expensive_calculation(amount: u32) -> u32 {
    println!("Perfoming calculation...");
    thread::sleep(Duration::from_secs(2));
    amount
}

struct Cacher<T>
    where
        T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
