
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn static_function() -> String {
        let s1 = String::from("static function");
        return s1;
    }
}

fn main() {
    
    let r1 = Rectangle{
        height: 10,
        width: 20
    };

    println!("\nHeight of r1 is {} and width is {}", r1.height, r1.width);
    println!("\nArea of r1 is {}", r1.area());
    
    let response = Rectangle::static_function();
    println!("\n{}\n", response);

}


