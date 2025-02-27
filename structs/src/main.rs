use rectangle::Rectangle;
pub mod rectangle;

fn main() {
    
    let r1 = Rectangle{
        length: 10,
        width: 20
    };

    println!("\nHeight of r1 is {} and width is {}", r1.length, r1.width);
    println!("\nArea of r1 is {}", r1.area());
    
    let response = Rectangle::static_function();
    println!("\n{}\n", response);

}


