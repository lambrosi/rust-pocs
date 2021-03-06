fn main() {

    // Create a new vector
    // To be able to push values, the instance must be mutable
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // Create a new vector with initial values
    // Same result as the previous, but this macro is more convenient
    let v2 = vec![1, 2, 3];

    // ========================================

    // Read values
    let v3 = vec![1, 2, 3, 4, 5];

    // Panics if index not exist
    let third: &i32 = &v3[2];
    println!("Third element: {}.", third);

    // Return 'None' if index not exist
    match v3.get(2) {
        Some(third) => println!("Third element: {}.", third),
        None => println!("Third element not found!")
    }

    // ========================================

    // Iterate over immutable items
    for i in &v3 {
        println!("Value: {}", i);
    }

    // Iterate over mutable items
    // * is te dereference operator
    let mut v4 = vec![40, 70, 15];
    for value in &mut v4 {
        *value += 50;
    }

    // ========================================

    // Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
