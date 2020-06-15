fn main() {

    // Iterators are lazy by default
    // This two lines do nothing useful
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // --------------------------

    let v2 = vec![1, 2, 3];

    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
    println!("{:?}", v3);

    // --------------------------

    let shoe1 = Shoe { size: 40, style: "Shoe 1".to_string()};
    let shoe2 = Shoe { size: 39, style: "Shoe 2".to_string()};
    let shoe3 = Shoe { size: 40, style: "Shoe 3".to_string()};
    let shoes = vec![shoe1, shoe2, shoe3];

    println!("Shoes: {:?}", shoes_in_my_size(shoes, 40)); // List: Shoe1 and Shoe2
}

fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == size)
        .collect()
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String
}
