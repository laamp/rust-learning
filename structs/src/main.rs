fn main() {
    let rect1 = Rectangle {
        color: "green".to_string(),
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        color: "red".to_string(),
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        color: "blue".to_string(),
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    color: String,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        (self.width > other_rect.width) && (self.height > other_rect.height)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn get_color(&self) -> &str {
        &self.color
    }
}
