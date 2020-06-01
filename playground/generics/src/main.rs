struct Point<T> {
    x: T,
    y: T
}

//impl<T> because this function is for T
//functions can be implemented for only a certain type, ex: impl<f32>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {

    let integer = Point { x: 10, y: 7 };
    let float = Point { x: 2.5, y: 6.7 };

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: Copy + std::cmp::PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}