fn main() {
    // let width1 = 10;
    // let height1 = 10;

    // let rect = (2, 75);

    let rect = Rectangle {
        width: 7,
        height: 7,
    };

    // println!("The area of the rectangle is {}", area(width1, height1));

    // println!("The area of the rectangle is {}", area(rect));

    println!("The area of the rectangle is {}", area(&rect));

    println!("{:#?}", rect);
}

// fn area(w: u32, h: u32) -> u32 {
//     w * h
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
