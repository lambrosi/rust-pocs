use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let value = 10;

    print_result(value, true);
    println!();
    print_result(3, false);
}

fn print_result(intensity: u32, standard: bool) {
    // If the expression have multiple lines, use {}
    // let expensive_closure = |number| {
    //     let n = number 2;
    //     n * 4
    // };

    let expensive_closure = Cacher::new(|num| expensive_calculation(num));

    if intensity > 5 {
        let result = expensive_closure.value(intensity);
        println!("First opt: {}", result);
        println!("Second opt: {}", result);
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
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                println!("{}", v);
                self.values.insert(arg, v);
                *self.values.get(&arg).unwrap()
            }
        }
    }
}
