fn main() {
    //IF expression
    let number = 3;

    if number == 5 {
        println!("The number is equals than 5.");
    } else if number > 5 {
        println!("The number is greater than 5.");
    } else {
        println!("The number is less than 5.");
    }


    //Using if in a let Statement
    let condition = true;
    let num = if condition { 10 } else { 5 };

    println!("The value of num is {}.", num);
}
