use add_one;
use add_two;

fn main() {
    let x = 3;
    println!("{} plus one is {}", x, add_one::add_one(3));

    println!("{} plus two is {}", x, add_two::add_two(3));
}
