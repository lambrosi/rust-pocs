fn main() {

    // Closures can use variables declared in the same scope

    let x = 4;

    let equal_to_x = |y| y == x;

    println!("{}", equal_to_x(4)); // true
}
