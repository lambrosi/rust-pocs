fn main() {
    //Ways to initialize an array
    let a1 = [1, 2, 3, 4, 5];
    let a2: [u32; 5] = [1, 2, 3, 4, 5];
    let a3 = [3; 7];      // ---> this equals to a3 = [3, 3, 3, 3, 3, 3, 3];

    println!("Size of a1 is {}.", a1.len());
    println!("Second value of a2 is {}.", a2[1]);
    println!("Size of a3 is 7: {}.", a3.len() == 7);
    
}
