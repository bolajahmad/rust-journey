fn main() {
    let rectangle = Rectangle {
        width: dbg!(20),
        height: 5,
    };
    // dbg!(&rectangle);
    println!("rectangle is {:#?}", rectangle);
    calculate_area(&rectangle);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
