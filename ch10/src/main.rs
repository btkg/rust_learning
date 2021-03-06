#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    println!("{:#?}, x = {}", integer, integer.x());
    let float = Point { x: 1.0, y: 2.0 };
    println!("{:#?}", float);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
