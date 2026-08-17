#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 30
    };

    println!(
        "A área do retângulo é {} pixels quadrados",
        area(&rect1)
    );

    println!("{:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
