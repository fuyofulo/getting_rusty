
mod implementation;
use crate::implementation::Shape;

fn main() {
    
    let s1 = Shape::Square(10.0);
    let s2 = Shape::Circle(5.0);
    let s3 = Shape::Rectangle(3.0, 4.0);

    println!("\nArea of s1 is {}\n", s1.area());
    println!("\nArea of s2 is {}\n", s2.area());
    println!("\nArea of s3 is {}\n", s3.area());

}
