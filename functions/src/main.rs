fn main() {
    println!("Hello, world!");

    another_function(101, 9);
}

fn another_function(num1: i32, num2: i32) {
    println!("Another function received {} and {}.", num1, num2);
}
